//! Eager immutable-chunkmap reuse experiment; this is not a production format.
//!
//! The structural API is pinned to immutable-chunkmap 2.1.4, upstream revision
//! e600c70f70a79df05fb38dc4a74aa919dd399d2b. Addresses only memoize live nodes:
//! every memo entry owns a NodeHandle that pins the address. Durable references
//! are the checksum IDs returned by Objects. No upstream tree code is modified.

use crate::{Id, Objects, Result};
use immutable_chunkmap::map::{Map, NodeHandle, NodeRef};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

const CHUNK: usize = 64;
const FORMAT: &str = "chunkmap64-v1";
const MAX_OBJECT_BYTES: usize = crate::MAX_OBJECT;
const MAX_KEY_BYTES: usize = 1024;
const MAX_VALUE_BYTES: usize = 1024;
const MAX_DEPTH: usize = 64;
const MAX_NODES: usize = 1_000_000;
type Tree = Map<String, Vec<u8>, CHUNK>;
type Handle = NodeHandle<String, Vec<u8>, CHUNK>;
type View<'a> = NodeRef<'a, String, Vec<u8>, CHUNK>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct NodeRecord {
    format: String,
    pairs: Vec<(String, Vec<u8>)>,
    left: Option<Id>,
    right: Option<Id>,
    height: u8,
    size: u64,
}

struct Pinned {
    id: Id,
    _handle: Handle,
    height: u8,
    size: u64,
}

#[derive(Clone)]
struct Loaded {
    handle: Handle,
    min: String,
    max: String,
    height: u8,
    size: u64,
}

/// Clone preserves an in-memory snapshot in O(1). The shared encoder memo pins
/// historical encoded nodes for this live lineage. Reopening starts a new memo
/// containing only the selected root's nodes. This deliberate no-reclamation
/// implementation therefore has an additional resident-history memory cost.
#[derive(Clone)]
pub struct ChunkIndex {
    map: Tree,
    memo: Arc<Mutex<HashMap<usize, Pinned>>>,
    root: Option<Id>,
    dirty: bool,
}

fn check_id(id: &str) -> Result<()> {
    if id.len() != 64
        || !id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("chunkmap: invalid durable object ID".into());
    }
    Ok(())
}

fn check_pair(key: &str, value: &[u8]) -> Result<()> {
    if key.len() > MAX_KEY_BYTES || value.len() > MAX_VALUE_BYTES {
        return Err("chunkmap: key/value resource limit exceeded".into());
    }
    Ok(())
}

impl ChunkIndex {
    pub fn open(store: &mut Objects, root: Option<Id>) -> Result<Self> {
        let mut memo = HashMap::new();
        let handle = if let Some(id) = root.as_deref() {
            Some(
                decode(
                    store,
                    id,
                    &mut HashMap::new(),
                    &mut HashSet::new(),
                    &mut memo,
                    &mut 0,
                    1,
                )?
                .handle,
            )
        } else {
            None
        };
        Ok(Self {
            map: Tree::from_root(handle),
            memo: Arc::new(Mutex::new(memo)),
            root,
            dirty: false,
        })
    }

    pub fn bulk(_store: &mut Objects, mut pairs: Vec<(String, Vec<u8>)>) -> Result<Self> {
        for (key, value) in &pairs {
            check_pair(key, value)?;
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        if pairs.windows(2).any(|p| p[0].0 >= p[1].0) {
            return Err("chunkmap: duplicate bulk key".into());
        }
        Ok(Self {
            map: pairs.into_iter().collect(),
            memo: Arc::new(Mutex::new(HashMap::new())),
            root: None,
            dirty: true,
        })
    }

    pub fn get(&mut self, _store: &mut Objects, key: &str) -> Result<Option<Vec<u8>>> {
        if key.len() > MAX_KEY_BYTES {
            return Err("chunkmap: lookup key resource limit exceeded".into());
        }
        Ok(self.map.get(key).cloned())
    }

    pub fn put(&mut self, _store: &mut Objects, key: String, value: Vec<u8>) -> Result<()> {
        check_pair(&key, &value)?;
        self.map.insert_cow(key, value);
        self.dirty = true;
        Ok(())
    }

    pub fn persist(&mut self, store: &mut Objects) -> Result<Option<Id>> {
        if !self.dirty {
            return Ok(self.root.clone());
        }
        let mut memo = self
            .memo
            .lock()
            .map_err(|_| "chunkmap: poisoned codec memo")?;
        self.root = encode(store, self.map.root(), &mut memo, 1)?.map(|n| n.0);
        self.dirty = false;
        Ok(self.root.clone())
    }

    /// Re-read and validate all objects positively reachable from this root.
    /// Unreferenced objects and the native sibling namespace are audited by the
    /// common harness, not inferred from the live in-memory map.
    pub fn audit(&mut self, store: &mut Objects) -> Result<usize> {
        let root = self.persist(store)?;
        Ok(Self::open(store, root)?.map.len())
    }
}

// Returns durable ID, structural height, and total subtree pair count. A memo hit
// stops traversal: encoding a one-key update visits only the changed path and the
// boundary shared children. The pinned handle prevents address reuse collisions.
fn encode(
    store: &mut Objects,
    node: Option<View<'_>>,
    memo: &mut HashMap<usize, Pinned>,
    depth: usize,
) -> Result<Option<(Id, u8, u64)>> {
    let Some(node) = node else { return Ok(None) };
    if depth > MAX_DEPTH {
        return Err("chunkmap: encoded depth limit exceeded".into());
    }
    let address = node.identity();
    if let Some(prior) = memo.get(&address) {
        return Ok(Some((prior.id.clone(), prior.height, prior.size)));
    }
    let left = encode(store, node.left(), memo, depth + 1)?;
    let right = encode(store, node.right(), memo, depth + 1)?;
    let lh = left.as_ref().map_or(0, |n| n.1);
    let rh = right.as_ref().map_or(0, |n| n.1);
    if lh.abs_diff(rh) > 2 {
        return Err("chunkmap: live tree balance invariant failed".into());
    }
    let height = lh
        .max(rh)
        .checked_add(1)
        .ok_or("chunkmap: height overflow")?;
    let size = left
        .as_ref()
        .map_or(0, |n| n.2)
        .checked_add(right.as_ref().map_or(0, |n| n.2))
        .and_then(|n| n.checked_add(node.len() as u64))
        .ok_or("chunkmap: size overflow")?;
    let pairs: Vec<_> = node.pairs().map(|(k, v)| (k.clone(), v.clone())).collect();
    let record = NodeRecord {
        format: FORMAT.to_owned(),
        pairs,
        left: left.map(|n| n.0),
        right: right.map(|n| n.0),
        height,
        size,
    };
    check_record(&record)?;
    let bytes = serde_json::to_vec(&record)?;
    if bytes.len() > MAX_OBJECT_BYTES {
        return Err("chunkmap: encoded node byte limit exceeded".into());
    }
    let id = store.write(&bytes)?;
    memo.insert(
        address,
        Pinned {
            id: id.clone(),
            _handle: node.keep(),
            height,
            size,
        },
    );
    Ok(Some((id, height, size)))
}

fn check_record(record: &NodeRecord) -> Result<()> {
    if record.format != FORMAT
        || record.pairs.is_empty()
        || record.pairs.len() > CHUNK
        || record.height == 0
        || record.height as usize > MAX_DEPTH
        || record.size > (MAX_NODES * CHUNK) as u64
    {
        return Err("chunkmap: node schema/resource bound violated".into());
    }
    for (key, value) in &record.pairs {
        check_pair(key, value)?;
    }
    if record.pairs.windows(2).any(|p| p[0].0 >= p[1].0) {
        return Err("chunkmap: node keys are not strictly ordered".into());
    }
    for id in record.left.iter().chain(record.right.iter()) {
        check_id(id)?;
    }
    Ok(())
}

fn decode(
    store: &mut Objects,
    id: &str,
    loaded: &mut HashMap<Id, Loaded>,
    visiting: &mut HashSet<Id>,
    memo: &mut HashMap<usize, Pinned>,
    node_count: &mut usize,
    depth: usize,
) -> Result<Loaded> {
    check_id(id)?;
    if depth > MAX_DEPTH {
        return Err("chunkmap: decode depth limit exceeded".into());
    }
    if let Some(prior) = loaded.get(id) {
        return Ok(prior.clone());
    }
    if !visiting.insert(id.to_owned()) {
        return Err("chunkmap: cyclic object graph".into());
    }
    *node_count += 1;
    if *node_count > MAX_NODES {
        return Err("chunkmap: eager decode node budget exceeded".into());
    }
    let bytes = store.read(id)?;
    if bytes.len() > MAX_OBJECT_BYTES {
        return Err("chunkmap: decoded node byte limit exceeded".into());
    }
    // Struct decoding rejects unknown and repeated fields; bounded exact bytes
    // also bound allocation before structural checks. No full-map Serde codec.
    let record: NodeRecord = serde_json::from_slice(&bytes)?;
    check_record(&record)?;
    let left = record
        .left
        .as_deref()
        .map(|child| decode(store, child, loaded, visiting, memo, node_count, depth + 1))
        .transpose()?;
    let right = record
        .right
        .as_deref()
        .map(|child| decode(store, child, loaded, visiting, memo, node_count, depth + 1))
        .transpose()?;
    let first = &record.pairs[0].0;
    let last = &record.pairs[record.pairs.len() - 1].0;
    if left.as_ref().is_some_and(|l| l.max >= *first)
        || right.as_ref().is_some_and(|r| r.min <= *last)
    {
        return Err("chunkmap: subtree key bounds violated".into());
    }
    let lh = left.as_ref().map_or(0, |n| n.height);
    let rh = right.as_ref().map_or(0, |n| n.height);
    let height = lh
        .max(rh)
        .checked_add(1)
        .ok_or("chunkmap: height overflow")?;
    let size = left
        .as_ref()
        .map_or(0, |n| n.size)
        .checked_add(right.as_ref().map_or(0, |n| n.size))
        .and_then(|n| n.checked_add(record.pairs.len() as u64))
        .ok_or("chunkmap: size overflow")?;
    if lh.abs_diff(rh) > 2 || height != record.height || size != record.size {
        return Err("chunkmap: height/balance/size invariant violated".into());
    }
    let min = left
        .as_ref()
        .map_or_else(|| first.clone(), |n| n.min.clone());
    let max = right
        .as_ref()
        .map_or_else(|| last.clone(), |n| n.max.clone());
    // SAFETY: All descendant handles have passed these same checks. This node
    // contains 1..=64 strictly ordered pairs; its full left/right subtrees lie
    // strictly below/above them; subtree heights differ by at most TWO as the
    // pinned upstream relaxed-AVL contract requires. Resource limits hold.
    // Source: upstream src/avl.rs NodeHandle::create at the revision above.
    let handle = unsafe {
        Handle::create(
            left.map(|n| n.handle),
            record.pairs,
            right.map(|n| n.handle),
        )
    };
    memo.insert(
        handle.view().identity(),
        Pinned {
            id: id.to_owned(),
            _handle: handle.clone(),
            height,
            size,
        },
    );
    let result = Loaded {
        handle,
        min,
        max,
        height,
        size,
    };
    visiting.remove(id);
    loaded.insert(id.to_owned(), result.clone());
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        path: PathBuf,
    }
    impl Fixture {
        fn new() -> (Self, Objects) {
            let path = std::env::temp_dir().join(format!(
                "membership-chunk-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            let objects = Objects::new(path.clone()).unwrap();
            (Self { path }, objects)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
    fn leaf(key: &str) -> NodeRecord {
        NodeRecord {
            format: FORMAT.into(),
            pairs: vec![(key.into(), vec![7])],
            left: None,
            right: None,
            height: 1,
            size: 1,
        }
    }
    fn rejected(store: &mut Objects, record: &NodeRecord) {
        let id = store.write(&serde_json::to_vec(record).unwrap()).unwrap();
        assert!(ChunkIndex::open(store, Some(id)).is_err());
    }

    #[test]
    fn randomized_model_roundtrips_and_snapshot_mutation() {
        let (_fixture, mut store) = Fixture::new();
        let mut index = ChunkIndex::open(&mut store, None).unwrap();
        let mut model = BTreeMap::new();
        let mut seed = 0x79c9_65b8_5273_50adu64;
        for step in 0..512u32 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let key = format!("key{:04}", seed % 257);
            let value = step.to_le_bytes().to_vec();
            let mut snapshot = index.clone();
            let before = snapshot.get(&mut store, &key).unwrap();
            index.put(&mut store, key.clone(), value.clone()).unwrap();
            model.insert(key.clone(), value.clone());
            assert_eq!(index.get(&mut store, &key).unwrap(), Some(value));
            assert_eq!(snapshot.get(&mut store, &key).unwrap(), before);
            if step % 64 == 63 {
                let root = index.persist(&mut store).unwrap();
                index = ChunkIndex::open(&mut store, root).unwrap();
                for (k, v) in &model {
                    assert_eq!(index.get(&mut store, k).unwrap().as_ref(), Some(v));
                }
                assert_eq!(index.get(&mut store, "never-present").unwrap(), None);
                assert_eq!(index.audit(&mut store).unwrap(), model.len());
            }
        }
    }

    #[test]
    fn incremental_codec_reuses_pinned_nodes_and_retains_old_root() {
        let (_fixture, mut store) = Fixture::new();
        let pairs = (0..4096)
            .map(|i| (format!("key{i:06}"), vec![42]))
            .collect();
        let mut index = ChunkIndex::bulk(&mut store, pairs).unwrap();
        let old_root = index.persist(&mut store).unwrap();
        let before = index.memo.lock().unwrap().len();
        index.put(&mut store, "key002048".into(), vec![99]).unwrap();
        let new_root = index.persist(&mut store).unwrap();
        let memo = index.memo.lock().unwrap();
        let added = memo.len() - before;
        assert!(
            added > 0 && added < before,
            "{added} new nodes for {before} prior nodes"
        );
        for (address, pinned) in memo.iter() {
            assert_eq!(*address, pinned._handle.view().identity());
        }
        drop(memo);
        assert_ne!(old_root, new_root);
        assert_eq!(index.persist(&mut store).unwrap(), new_root);
        let mut old = ChunkIndex::open(&mut store, old_root).unwrap();
        assert_eq!(old.get(&mut store, "key002048").unwrap(), Some(vec![42]));
        let mut new = ChunkIndex::open(&mut store, new_root).unwrap();
        assert_eq!(new.get(&mut store, "key002048").unwrap(), Some(vec![99]));
        assert_eq!(new.audit(&mut store).unwrap(), 4096);
    }

    #[test]
    fn rejects_malformed_shapes_order_bounds_heights_and_missing_nodes() {
        let (_fixture, mut store) = Fixture::new();
        let mut record = leaf("b");
        record.pairs.clear();
        rejected(&mut store, &record);
        record = leaf("b");
        record.pairs.push(("b".into(), vec![8]));
        rejected(&mut store, &record);
        record = leaf("b");
        record.pairs.push(("a".into(), vec![8]));
        rejected(&mut store, &record);
        record = leaf("b");
        record.height = 2;
        rejected(&mut store, &record);
        record = leaf("b");
        record.size = 2;
        rejected(&mut store, &record);
        record = leaf("b");
        record.format = "wrong-v1".into();
        rejected(&mut store, &record);
        record = leaf("b");
        record.pairs = (0..65).map(|i| (format!("k{i:03}"), vec![])).collect();
        record.size = 65;
        rejected(&mut store, &record);
        record = leaf("b");
        record.pairs[0].1 = vec![0; MAX_VALUE_BYTES + 1];
        rejected(&mut store, &record);
        let bad_child = store
            .write(&serde_json::to_vec(&leaf("z")).unwrap())
            .unwrap();
        record = leaf("b");
        record.left = Some(bad_child);
        record.height = 2;
        record.size = 2;
        rejected(&mut store, &record);
        record = leaf("b");
        record.left = Some("f".repeat(64));
        record.height = 2;
        record.size = 2;
        rejected(&mut store, &record);
        for bytes in [
            br#"{"format":"chunkmap64-v1","format":"chunkmap64-v1","pairs":[["b",[7]]],"left":null,"right":null,"height":1,"size":1}"#.as_slice(),
            br#"{"format":"chunkmap64-v1","pairs":[["b",[7]]],"left":null,"right":null,"height":1,"size":1,"unknown":0}"#.as_slice(),
            br#"{"format":"chunkmap64-v1","pairs":[["b",[7]]],"left":null,"right":null,"height":1,"size":1} {}"#.as_slice(),
        ] {
            let id = store.write(bytes).unwrap();
            assert!(ChunkIndex::open(&mut store, Some(id)).is_err());
        }
    }

    #[test]
    fn rejects_unbalanced_subtree_and_bad_input_without_mutation() {
        let (_fixture, mut store) = Fixture::new();
        let a = store
            .write(&serde_json::to_vec(&leaf("a")).unwrap())
            .unwrap();
        let mut b = leaf("b");
        b.left = Some(a);
        b.height = 2;
        b.size = 2;
        let b = store.write(&serde_json::to_vec(&b).unwrap()).unwrap();
        let mut c = leaf("c");
        c.left = Some(b);
        c.height = 3;
        c.size = 3;
        let c = store.write(&serde_json::to_vec(&c).unwrap()).unwrap();
        // Heights 3 and 0 violate relaxed AVL's difference <= 2.
        let mut d = leaf("d");
        d.left = Some(c);
        d.height = 4;
        d.size = 4;
        rejected(&mut store, &d);
        assert!(ChunkIndex::bulk(
            &mut store,
            vec![("same".into(), vec![1]), ("same".into(), vec![2])]
        )
        .is_err());
        let mut index = ChunkIndex::open(&mut store, None).unwrap();
        assert!(index
            .put(&mut store, "x".into(), vec![0; MAX_VALUE_BYTES + 1])
            .is_err());
        assert_eq!(index.get(&mut store, "x").unwrap(), None);
        assert_eq!(index.persist(&mut store).unwrap(), None);
        assert!(ChunkIndex::open(&mut store, Some("../invalid".into())).is_err());
    }
}
