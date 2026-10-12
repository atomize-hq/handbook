//! Application-owned transaction shell, inspired by EchoDB 0.8.0's small
//! begin/get/set/conditional/commit/cancel interface and closed/write checks.
//! Source: surrealdb/echodb revision 17b41ae82f3deba4b7d3034b0c2206a688c29f35,
//! src/{db,tx}.rs (Apache-2.0). No source lines are copied or adapted.
//! Unlike that in-memory shell, logical work does not hold the writer lock.
//! Durable publication, whole-root conflicts and native checks are new code.
use crate::{
    checkpoint, directory, hash_frame, identity, load_root, publish, sync_publication, File, Id,
    Index, Objects, Result, Root,
};
use fs2::FileExt;
use std::{collections::BTreeMap, fs::OpenOptions, time::Instant};

/// Retained coordination handles for one established private store.
pub(crate) struct Database {
    variant: String,
    owner: String,
    lock_id: String,
    _directory: File,
    lock: File,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Commit {
    Published(u64),
    Unchanged(u64),
    Conflict(u64),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Publication {
    NotPublished,
    Indeterminate,
}

/// Failed commit outcome belongs to this attempt, even if Objects is reused
/// after an earlier successful publication. Never infer rollback from IO error.
#[derive(Debug)]
pub(crate) struct CommitFailure {
    pub(crate) publication: Publication,
    source: Box<dyn std::error::Error>,
}
impl std::fmt::Display for CommitFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.source)
    }
}
impl std::error::Error for CommitFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

impl Database {
    pub(crate) fn open(o: &mut Objects, variant: &str) -> Result<Self> {
        let dir = directory(&o.base)?;
        let owner = identity(&dir)?;
        if o.read_record(&o.base.join("OWNER"), 256)? != owner.as_bytes() {
            return Err("native root identity changed".into());
        }
        let path = o.base.join("LOCK");
        o.guard(&path)?;
        // Never recreate missing coordination evidence.
        let lock = OpenOptions::new().read(true).write(true).open(path)?;
        let lock_id = identity(&lock)?;
        Ok(Self {
            variant: variant.into(),
            owner,
            lock_id,
            _directory: dir,
            lock,
        })
    }

    pub(crate) fn begin(&self, o: &mut Objects, write: bool) -> Result<Transaction> {
        let (root, expected) = load_root(o, &self.variant, &self.owner)?;
        if root.lock != self.lock_id {
            return Err("native established lock identity changed".into());
        }
        let index = Index::open(o, &self.variant, root.node.clone())?;
        Ok(Transaction {
            root,
            expected,
            index,
            write,
            closed: false,
            overlay: BTreeMap::new(),
            dependencies: BTreeMap::new(),
        })
    }
}

/// Immutable base snapshot plus bounded application-owned logical updates.
/// No staging IO during put/cancel/drop. No external callback or automatic retry.
pub(crate) struct Transaction {
    pub(crate) root: Root,
    expected: Id,
    pub(crate) index: Index,
    write: bool,
    closed: bool,
    overlay: BTreeMap<String, Vec<u8>>,
    dependencies: BTreeMap<String, Option<Vec<u8>>>,
}

impl Transaction {
    fn active(&self) -> Result<()> {
        if self.closed {
            return Err("closed transaction".into());
        }
        Ok(())
    }

    pub(crate) fn get(&mut self, o: &mut Objects, key: &str) -> Result<Option<Vec<u8>>> {
        self.active()?;
        if key.len() > 1024 {
            return Err("transaction key bound".into());
        }
        if let Some(value) = self.overlay.get(key) {
            return Ok(Some(value.clone()));
        }
        let value = self.index.get(o, key)?;
        if !self.dependencies.contains_key(key) && self.dependencies.len() >= 4096 {
            return Err("transaction point dependency bound".into());
        }
        self.dependencies
            .entry(key.into())
            .or_insert_with(|| value.clone());
        Ok(value)
    }

    pub(crate) fn put(&mut self, key: String, value: Vec<u8>) -> Result<()> {
        self.active()?;
        if !self.write {
            return Err("read-only transaction".into());
        }
        if key.len() > 1024 || value.len() > 1024 {
            return Err("transaction pair bound".into());
        }
        if !self.overlay.contains_key(&key) && self.overlay.len() >= 256 {
            return Err("transaction buffered pair bound".into());
        }
        self.overlay.insert(key, value);
        Ok(())
    }

    pub(crate) fn put_if(
        &mut self,
        o: &mut Objects,
        key: String,
        value: Vec<u8>,
        expected: Option<Vec<u8>>,
    ) -> Result<()> {
        if self.get(o, &key)? != expected {
            return Err("conditional value differs".into());
        }
        self.put(key, value)
    }

    pub(crate) fn cancel(&mut self) -> Result<()> {
        self.active()?;
        self.closed = true;
        self.overlay.clear();
        Ok(())
    }

    pub(crate) fn buffered(&self) -> (u64, u64) {
        (
            self.overlay.len() as u64,
            self.overlay
                .iter()
                .map(|(k, v)| (k.len() + v.len()) as u64)
                .sum(),
        )
    }

    pub(crate) fn commit(
        &mut self,
        db: &Database,
        o: &mut Objects,
        coalesce: bool,
        replay: bool,
    ) -> std::result::Result<Commit, CommitFailure> {
        o.publication_possible = false;
        self.commit_inner(db, o, coalesce, replay)
            .map_err(|source| CommitFailure {
                publication: if o.publication_possible {
                    Publication::Indeterminate
                } else {
                    Publication::NotPublished
                },
                source,
            })
    }

    fn commit_inner(
        &mut self,
        db: &Database,
        o: &mut Objects,
        coalesce: bool,
        replay: bool,
    ) -> Result<Commit> {
        self.active()?;
        // All commit outcomes close this instance. Cancel cannot undo publication.
        self.closed = true;
        if !self.write {
            if replay {
                return Err("terminal replay requires coordinated publication repair".into());
            }
            return Ok(Commit::Unchanged(self.root.seq));
        }
        let wait = Instant::now();
        db.lock.lock_exclusive()?;
        o.metrics.lock_wait_ns += wait.elapsed().as_nanos() as u64;
        let held = Instant::now();
        let result = (|| {
            o.guard(&o.base.clone())?;
            if identity(&directory(&o.base)?)? != db.owner
                || identity(&File::open(o.base.join("LOCK"))?)? != db.lock_id
                || o.read_record(&o.base.join("OWNER"), 256)? != db.owner.as_bytes()
            {
                return Err("native root/lock identity changed".into());
            }
            let (current, id) = load_root(o, &db.variant, &db.owner)?;
            if current.lock != db.lock_id {
                return Err("native selected lock identity changed".into());
            }
            if id != self.expected {
                o.metrics.retries += 1;
                return Ok(Commit::Conflict(current.seq));
            }
            // Exact-root validation is conservative. This point recheck detects
            // dependency errors; it is not read-set OCC or a disjoint merge.
            for (key, value) in &self.dependencies {
                if self.index.get(o, key)? != *value {
                    return Err("snapshot dependency changed".into());
                }
            }
            if self.overlay.is_empty() {
                if replay {
                    o.publication_possible = true;
                    sync_publication(o)?;
                    if o.instrument {
                        o.metrics.replay_repairs += 1;
                    }
                    checkpoint("replay_sync")?;
                }
                return Ok(Commit::Unchanged(self.root.seq));
            }
            let pairs = self
                .overlay
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            self.index.put_batch(o, pairs, coalesce)?;
            let next = Root {
                format: 1,
                variant: db.variant.clone(),
                seq: self.root.seq.checked_add(1).ok_or("sequence overflow")?,
                node: self.index.persist(o)?,
                parent: Some(hash_frame(&serde_json::to_vec(&self.root)?)),
                owner: db.owner.clone(),
                lock: db.lock_id.clone(),
            };
            publish(o, &next, Some(&self.expected))?;
            self.root = next;
            Ok(Commit::Published(self.root.seq))
        })();
        o.metrics.lock_hold_ns += held.elapsed().as_nanos() as u64;
        // Always release coordination after the attempt, including known failure.
        let unlocked = fs2::FileExt::unlock(&db.lock);
        match result {
            Err(error) => Err(error),
            Ok(value) => {
                unlocked?;
                Ok(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::execute;
    use serde_json::json;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn established(variant: &str) -> Objects {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "membership-tx-test-{}-{}-{}",
            std::process::id(),
            variant,
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        let mut o = Objects::new(path).unwrap();
        execute(&mut o, variant, &json!({"op":"init","n":64})).unwrap();
        o.metrics = crate::Metrics::default();
        o.publication_possible = false;
        o
    }

    #[test]
    fn snapshot_overlay_atomic_commit_and_closed_lifecycle() {
        for variant in ["chunk", "disk"] {
            let mut o = established(variant);
            let db = Database::open(&mut o, variant).unwrap();
            let mut reader = db.begin(&mut o, false).unwrap();
            let mut tx = db.begin(&mut o, true).unwrap();
            tx.put_if(&mut o, "u:first".into(), b"one".to_vec(), None)
                .unwrap();
            assert_eq!(tx.get(&mut o, "u:first").unwrap(), Some(b"one".to_vec()));
            tx.put_if(
                &mut o,
                "u:first".into(),
                b"two".to_vec(),
                Some(b"one".to_vec()),
            )
            .unwrap();
            tx.put("u:second".into(), b"pair".to_vec()).unwrap();
            assert_eq!(o.metrics.writes, 0); // actual buffered work, no object install
            assert_eq!(
                tx.commit(&db, &mut o, true, false).unwrap(),
                Commit::Published(1)
            );
            assert!(tx.cancel().is_err());
            assert!(tx.get(&mut o, "u:first").is_err());
            assert!(tx.commit(&db, &mut o, true, false).is_err());
            assert_eq!(reader.get(&mut o, "u:first").unwrap(), None); // read AFTER commit
            let mut fresh = db.begin(&mut o, false).unwrap();
            assert_eq!(fresh.get(&mut o, "u:first").unwrap(), Some(b"two".to_vec()));
            assert_eq!(
                fresh.get(&mut o, "u:second").unwrap(),
                Some(b"pair".to_vec())
            );
            assert!(fresh.put("u:bad".into(), vec![]).is_err());
            let mut invalid_replay = db.begin(&mut o, false).unwrap();
            let failure = invalid_replay.commit(&db, &mut o, true, true).unwrap_err();
            assert_eq!(failure.publication, Publication::NotPublished);
            assert!(failure
                .to_string()
                .contains("coordinated publication repair"));
            reader.cancel().unwrap();
            assert!(reader.cancel().is_err());
        }
    }

    #[test]
    fn explicit_root_conflict_cancel_drop_and_input_failure_preserve_membership() {
        for variant in ["chunk", "disk"] {
            let mut o = established(variant);
            let db = Database::open(&mut o, variant).unwrap();
            let mut stale = db.begin(&mut o, true).unwrap();
            stale.put("u:stale".into(), b"proposal".to_vec()).unwrap();
            let mut winner = db.begin(&mut o, true).unwrap();
            winner.put("u:winner".into(), b"retained".to_vec()).unwrap();
            winner.commit(&db, &mut o, true, false).unwrap();
            let writes = o.metrics.writes;
            assert_eq!(
                stale.commit(&db, &mut o, true, false).unwrap(),
                Commit::Conflict(1)
            );
            assert_eq!(o.metrics.writes, writes);
            assert!(stale.cancel().is_err());
            let mut cancel = db.begin(&mut o, true).unwrap();
            cancel.put("u:cancel".into(), vec![1]).unwrap();
            assert!(cancel.put("u:invalid".into(), vec![0; 1025]).is_err());
            assert!(cancel
                .put_if(&mut o, "u:winner".into(), vec![2], None)
                .is_err());
            cancel.cancel().unwrap();
            let mut dropped = db.begin(&mut o, true).unwrap();
            dropped.put("u:dropped".into(), vec![3]).unwrap();
            drop(dropped);
            let mut reader = db.begin(&mut o, false).unwrap();
            for key in ["u:stale", "u:cancel", "u:invalid", "u:dropped"] {
                assert_eq!(reader.get(&mut o, key).unwrap(), None);
            }
            assert_eq!(o.metrics.writes, writes);
        }
    }

    #[test]
    fn reused_store_reports_this_attempts_prepublication_failure() {
        for variant in ["chunk", "disk"] {
            let mut o = established(variant);
            let db = Database::open(&mut o, variant).unwrap();
            let mut winner = db.begin(&mut o, true).unwrap();
            winner.put("u:first".into(), b"retained".to_vec()).unwrap();
            winner.commit(&db, &mut o, true, false).unwrap();
            assert!(o.publication_possible);
            let mut tx = db.begin(&mut o, true).unwrap();
            tx.put("u:failed".into(), b"proposal".to_vec()).unwrap();
            // Positive native coordination loss, with a prior published attempt.
            std::fs::rename(o.base.join("LOCK"), o.base.join("LOCK-old")).unwrap();
            let failure = tx.commit(&db, &mut o, true, false).unwrap_err();
            assert_eq!(failure.publication, Publication::NotPublished);
            assert!(!o.publication_possible);
            assert!(tx.cancel().is_err());
        }
    }
}
