//! Experimental immutable disk B+tree. This module owns no publication or GC.
//!
//! Only the selected path is loaded by lookup/update; `audit` checks all reachable
//! objects. A changed path is written immediately, but remains an unpublished
//! proposal until the common harness publishes the returned root. Failed writes
//! may leave unreachable objects; the old `DiskIndex` root remains unchanged.
use crate::{Id, Objects, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const LEAF_CAPACITY: usize = 64;
const FANOUT: usize = 32;
const MAX_OBJECT_BYTES: usize = crate::MAX_OBJECT;
const MAX_BATCH_PAIRS: usize = 256;
const MAX_KEY_BYTES: usize = 1024;
const MAX_VALUE_BYTES: usize = 1024;
const MAX_HEIGHT: u8 = 63; // A root-to-leaf path has at most 64 objects.
const MAX_AUDIT_NODES: usize = 1_000_000;

// Closed fields and an explicit variant tag follow Serde's documented formats:
// https://serde.rs/container-attrs.html#deny_unknown_fields
// https://serde.rs/enum-representations.html#internally-tagged
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum Node {
    Leaf { pairs: Vec<(String, Vec<u8>)> },
    Branch { height: u8, children: Vec<Child> },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Child {
    id: Id,
    min: String,
    max: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Bounds {
    height: u8,
    min: String,
    max: String,
}

#[derive(Clone)]
struct Written {
    child: Child,
    height: u8,
}

/// Cheap snapshots retain immutable root IDs. Old readers/proposals require the
/// common store's no-reclamation policy; this type alone is not a reachability GC.
#[derive(Clone, Debug)]
pub struct DiskIndex {
    root: Option<Id>,
}

fn valid_id(id: &str) -> bool {
    id.len() == 64
        && id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn validate_pair(key: &str, value: &[u8]) -> Result<()> {
    if key.len() > MAX_KEY_BYTES || value.len() > MAX_VALUE_BYTES {
        return Err("disk pair exceeds the experimental key/value bound".into());
    }
    Ok(())
}

fn bounds(node: &Node) -> Result<Bounds> {
    match node {
        Node::Leaf { pairs } => {
            if pairs.is_empty() || pairs.len() > LEAF_CAPACITY {
                return Err("disk leaf must contain 1..=64 pairs".into());
            }
            for (key, value) in pairs {
                validate_pair(key, value)?;
            }
            if pairs.windows(2).any(|w| w[0].0 >= w[1].0) {
                return Err("disk leaf keys are not strictly ordered".into());
            }
            Ok(Bounds {
                height: 0,
                min: pairs[0].0.clone(),
                max: pairs[pairs.len() - 1].0.clone(),
            })
        }
        Node::Branch { height, children } => {
            if *height == 0 || *height > MAX_HEIGHT || children.len() < 2 || children.len() > FANOUT
            {
                return Err("disk branch height/fanout is outside bounds".into());
            }
            for child in children {
                if !valid_id(&child.id)
                    || child.min.len() > MAX_KEY_BYTES
                    || child.max.len() > MAX_KEY_BYTES
                    || child.min > child.max
                {
                    return Err("disk branch has invalid child identity/bounds".into());
                }
            }
            if children.windows(2).any(|w| w[0].max >= w[1].min) {
                return Err("disk branch child ranges overlap or are unordered".into());
            }
            Ok(Bounds {
                height: *height,
                min: children[0].min.clone(),
                max: children[children.len() - 1].max.clone(),
            })
        }
    }
}

fn expected_child(child: &Child, height: u8) -> Bounds {
    Bounds {
        height,
        min: child.min.clone(),
        max: child.max.clone(),
    }
}

fn read_node(store: &mut Objects, id: &str, expected: Option<&Bounds>) -> Result<Node> {
    if !valid_id(id) {
        return Err("invalid disk node content ID".into());
    }
    let bytes = store.read(id)?;
    if bytes.len() > MAX_OBJECT_BYTES {
        return Err("disk node exceeds common object byte limit".into());
    }
    // from_slice consumes exactly one JSON document. Derive rejects duplicate
    // named fields; tests below also cover duplicate tag/field and trailing data.
    let node: Node = serde_json::from_slice(&bytes)?;
    let actual = bounds(&node)?;
    if expected.is_some_and(|expected| expected != &actual) {
        return Err("disk selected child does not match parent bounds/height".into());
    }
    Ok(node)
}

fn write_node(store: &mut Objects, node: Node) -> Result<Written> {
    let b = bounds(&node)?;
    let bytes = serde_json::to_vec(&node)?;
    if bytes.len() > MAX_OBJECT_BYTES {
        return Err("disk node exceeds common object byte limit".into());
    }
    let id = store.write(&bytes)?;
    Ok(Written {
        child: Child {
            id,
            min: b.min,
            max: b.max,
        },
        height: b.height,
    })
}

fn leaf_fits(pairs: &[(String, Vec<u8>)]) -> Result<bool> {
    if pairs.len() > LEAF_CAPACITY {
        return Ok(false);
    }
    // Also split by serialized byte size. Count bounds alone are insufficient
    // because byte-array JSON and escaped keys may expand substantially.
    Ok(serde_json::to_vec(&Node::Leaf {
        pairs: pairs.to_vec(),
    })?
    .len()
        <= MAX_OBJECT_BYTES)
}

fn write_leaves(store: &mut Objects, pairs: Vec<(String, Vec<u8>)>) -> Result<Vec<Written>> {
    if leaf_fits(&pairs)? {
        return Ok(vec![write_node(store, Node::Leaf { pairs })?]);
    }
    // A legal single pair is far smaller than one page. Finding a balanced legal
    // boundary allows every admitted value, including heavily escaped keys.
    let mut candidates: Vec<usize> = (1..pairs.len()).collect();
    candidates.sort_by_key(|n| n.abs_diff(pairs.len() / 2));
    for split in candidates {
        if leaf_fits(&pairs[..split])? && leaf_fits(&pairs[split..])? {
            return Ok(vec![
                write_node(
                    store,
                    Node::Leaf {
                        pairs: pairs[..split].to_vec(),
                    },
                )?,
                write_node(
                    store,
                    Node::Leaf {
                        pairs: pairs[split..].to_vec(),
                    },
                )?,
            ]);
        }
    }
    // Bulk input can exceed two pages even when every individual pair is valid.
    // Runtime batches may combine one old page with up to 256 bounded pairs.
    let split = pairs.len() / 2;
    if split == 0 {
        return Err("disk pair cannot fit within page limits".into());
    }
    let mut written = write_leaves(store, pairs[..split].to_vec())?;
    written.extend(write_leaves(store, pairs[split..].to_vec())?);
    Ok(written)
}

fn write_branch(store: &mut Objects, height: u8, children: Vec<Child>) -> Result<Vec<Written>> {
    let fits = |children: &[Child]| -> Result<bool> {
        Ok(children.len() <= FANOUT
            && serde_json::to_vec(&Node::Branch {
                height,
                children: children.to_vec(),
            })?
            .len()
                <= MAX_OBJECT_BYTES)
    };
    if fits(&children)? {
        return Ok(vec![write_node(store, Node::Branch { height, children })?]);
    }
    let mut candidates: Vec<usize> = (2..children.len().saturating_sub(1)).collect();
    candidates.sort_by_key(|n| n.abs_diff(children.len() / 2));
    for split in candidates {
        if fits(&children[..split])? && fits(&children[split..])? {
            return Ok(vec![
                write_node(
                    store,
                    Node::Branch {
                        height,
                        children: children[..split].to_vec(),
                    },
                )?,
                write_node(
                    store,
                    Node::Branch {
                        height,
                        children: children[split..].to_vec(),
                    },
                )?,
            ]);
        }
    }
    let split = children.len() / 2;
    if split < 2 || children.len() - split < 2 {
        return Err("disk branch cannot fit within page limits".into());
    }
    let mut written = write_branch(store, height, children[..split].to_vec())?;
    written.extend(write_branch(store, height, children[split..].to_vec())?);
    Ok(written)
}

fn put_path(
    store: &mut Objects,
    id: &str,
    expected: Option<&Bounds>,
    key: &str,
    value: &[u8],
    depth: usize,
) -> Result<Vec<Written>> {
    if depth >= usize::from(MAX_HEIGHT) + 1 {
        return Err("disk update path exceeds depth limit".into());
    }
    let node = read_node(store, id, expected)?;
    match node {
        Node::Leaf { mut pairs } => {
            match pairs.binary_search_by(|(k, _)| k.as_str().cmp(key)) {
                Ok(position) => {
                    if pairs[position].1 == value {
                        let b = bounds(&Node::Leaf { pairs })?;
                        return Ok(vec![Written {
                            child: Child {
                                id: id.to_owned(),
                                min: b.min,
                                max: b.max,
                            },
                            height: 0,
                        }]);
                    }
                    pairs[position].1 = value.to_vec();
                }
                Err(position) => pairs.insert(position, (key.to_owned(), value.to_vec())),
            }
            write_leaves(store, pairs)
        }
        Node::Branch {
            height,
            mut children,
        } => {
            let position = children
                .partition_point(|c| c.max.as_str() < key)
                .min(children.len() - 1);
            let old = children[position].clone();
            let replacement = put_path(
                store,
                &old.id,
                Some(&expected_child(&old, height - 1)),
                key,
                value,
                depth + 1,
            )?;
            // A no-op update leaves the whole path untouched.
            if replacement.len() == 1 && replacement[0].child.id == old.id {
                let b = bounds(&Node::Branch { height, children })?;
                return Ok(vec![Written {
                    child: Child {
                        id: id.to_owned(),
                        min: b.min,
                        max: b.max,
                    },
                    height,
                }]);
            }
            children.splice(
                position..=position,
                replacement.into_iter().map(|r| r.child),
            );
            write_branch(store, height, children)
        }
    }
}

// All updates are sorted, unique and validated before entering this traversal.
// Routing uses the original children's maxima, so disjoint subsets can be
// applied independently and then combined into one final parent write. Objects
// are still installed by the common protocol while its writer lock is held.
fn put_batch_path(
    store: &mut Objects,
    id: &str,
    expected: Option<&Bounds>,
    updates: &[(String, Vec<u8>)],
    depth: usize,
) -> Result<Vec<Written>> {
    if depth >= usize::from(MAX_HEIGHT) + 1 {
        return Err("disk batch update path exceeds depth limit".into());
    }
    let node = read_node(store, id, expected)?;
    let original = bounds(&node)?;
    let unchanged = Written {
        child: Child {
            id: id.to_owned(),
            min: original.min,
            max: original.max,
        },
        height: original.height,
    };
    match node {
        Node::Leaf { mut pairs } => {
            let mut changed = false;
            for (key, value) in updates {
                match pairs.binary_search_by(|(existing, _)| existing.cmp(key)) {
                    Ok(position) if pairs[position].1 == *value => {}
                    Ok(position) => {
                        pairs[position].1 = value.clone();
                        changed = true;
                    }
                    Err(position) => {
                        pairs.insert(position, (key.clone(), value.clone()));
                        changed = true;
                    }
                }
            }
            if changed {
                write_leaves(store, pairs)
            } else {
                Ok(vec![unchanged])
            }
        }
        Node::Branch { height, children } => {
            let mut final_children = Vec::with_capacity(children.len());
            let mut offset = 0;
            let mut changed = false;
            for (position, child) in children.iter().enumerate() {
                let end = if position + 1 == children.len() {
                    updates.len()
                } else {
                    offset + updates[offset..].partition_point(|(key, _)| key <= &child.max)
                };
                if end == offset {
                    final_children.push(child.clone());
                    continue;
                }
                let replacement = put_batch_path(
                    store,
                    &child.id,
                    Some(&expected_child(child, height - 1)),
                    &updates[offset..end],
                    depth + 1,
                )?;
                changed |= replacement.len() != 1 || replacement[0].child.id != child.id;
                final_children.extend(replacement.into_iter().map(|entry| entry.child));
                offset = end;
            }
            if changed {
                write_branch(store, height, final_children)
            } else {
                Ok(vec![unchanged])
            }
        }
    }
}

// A bounded update can split several sibling paths at once. Build only final
// upper levels, balancing groups to avoid a one-child branch (33 => 17 + 16).
fn finish_batch_root(store: &mut Objects, mut pages: Vec<Written>) -> Result<Id> {
    while pages.len() > 1 {
        let height = pages[0]
            .height
            .checked_add(1)
            .ok_or("disk batch height overflow")?;
        if height > MAX_HEIGHT {
            return Err("disk batch exceeds height limit".into());
        }
        let groups = pages.len().div_ceil(FANOUT);
        let width = pages.len() / groups;
        let extra = pages.len() % groups;
        let mut next = Vec::with_capacity(groups);
        let mut offset = 0;
        for group in 0..groups {
            let count = width + usize::from(group < extra);
            let children = pages[offset..offset + count]
                .iter()
                .map(|page| page.child.clone())
                .collect();
            next.extend(write_branch(store, height, children)?);
            offset += count;
        }
        pages = next;
    }
    Ok(pages.remove(0).child.id)
}

impl DiskIndex {
    /// Verify the selected root page; children stay lazy until a path/audit reads
    /// them. The root is not cached, so each lookup measures its root read too.
    pub fn open(store: &mut Objects, root: Option<Id>) -> Result<Self> {
        if let Some(id) = &root {
            read_node(store, id, None)?;
        }
        Ok(Self { root })
    }

    /// Initial bulk construction only. Runtime insertion always uses path COW.
    pub fn bulk(store: &mut Objects, mut pairs: Vec<(String, Vec<u8>)>) -> Result<Self> {
        if pairs.is_empty() {
            return Ok(Self { root: None });
        }
        for (key, value) in &pairs {
            validate_pair(key, value)?;
        }
        pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        if pairs.windows(2).any(|w| w[0].0 == w[1].0) {
            return Err("duplicate disk bulk key".into());
        }
        let mut pages = Vec::new();
        for leaf in pairs.chunks(LEAF_CAPACITY) {
            pages.extend(write_leaves(store, leaf.to_vec())?);
        }
        while pages.len() > 1 {
            let height = pages[0]
                .height
                .checked_add(1)
                .ok_or("disk height overflow")?;
            if height > MAX_HEIGHT {
                return Err("disk bulk exceeds height limit".into());
            }
            // Balance groups so no branch has a single child (33 => 17 + 16).
            let groups = pages.len().div_ceil(FANOUT);
            let width = pages.len() / groups;
            let extra = pages.len() % groups;
            let mut next = Vec::with_capacity(groups);
            let mut offset = 0;
            for group in 0..groups {
                let count = width + usize::from(group < extra);
                let children = pages[offset..offset + count]
                    .iter()
                    .map(|p| p.child.clone())
                    .collect();
                next.extend(write_branch(store, height, children)?);
                offset += count;
            }
            pages = next;
        }
        Ok(Self {
            root: Some(pages.remove(0).child.id),
        })
    }

    pub fn get(&mut self, store: &mut Objects, key: &str) -> Result<Option<Vec<u8>>> {
        if key.len() > MAX_KEY_BYTES {
            return Err("disk lookup key exceeds bound".into());
        }
        let Some(mut id) = self.root.clone() else {
            return Ok(None);
        };
        let mut expected = None;
        for _ in 0..=MAX_HEIGHT {
            match read_node(store, &id, expected.as_ref())? {
                Node::Leaf { pairs } => {
                    return Ok(pairs
                        .binary_search_by(|(k, _)| k.as_str().cmp(key))
                        .ok()
                        .map(|p| pairs[p].1.clone()));
                }
                Node::Branch { height, children } => {
                    // Negative lookup reaches a leaf too. Returning solely from
                    // the parent's range claims would skip validation of the
                    // selected dependency and could hide a malformed range.
                    let position = children
                        .partition_point(|c| c.max.as_str() < key)
                        .min(children.len() - 1);
                    let child = &children[position];
                    id = child.id.clone();
                    expected = Some(expected_child(child, height - 1));
                }
            }
        }
        Err("disk lookup path exceeds depth limit".into())
    }

    pub fn put(&mut self, store: &mut Objects, key: String, value: Vec<u8>) -> Result<()> {
        validate_pair(&key, &value)?;
        let root = match &self.root {
            None => {
                write_node(
                    store,
                    Node::Leaf {
                        pairs: vec![(key, value)],
                    },
                )?
                .child
                .id
            }
            Some(id) => {
                let mut replacement = put_path(store, id, None, &key, &value, 0)?;
                if replacement.len() == 1 {
                    replacement.remove(0).child.id
                } else {
                    let height = replacement[0]
                        .height
                        .checked_add(1)
                        .ok_or("disk height overflow")?;
                    write_node(
                        store,
                        Node::Branch {
                            height,
                            children: replacement.into_iter().map(|r| r.child).collect(),
                        },
                    )?
                    .child
                    .id
                }
            }
        };
        // On any error above the previous root remains selected by this index.
        self.root = Some(root);
        Ok(())
    }

    /// Apply at most 256 unique buffered updates as one immutable proposal.
    /// Each affected old path is traversed once and each final changed node is
    /// written once, including any required splits. The common owner must still
    /// validate/publish this root under its native lock. Error leaves this index's
    /// root unchanged, but installed unreachable objects may remain (no GC).
    pub fn put_batch(
        &mut self,
        store: &mut Objects,
        mut pairs: Vec<(String, Vec<u8>)>,
    ) -> Result<()> {
        if pairs.len() > MAX_BATCH_PAIRS {
            return Err("disk batch exceeds 256 pair bound".into());
        }
        for (key, value) in &pairs {
            validate_pair(key, value)?;
        }
        pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        if pairs.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            return Err("duplicate disk batch key; resolve in application buffer".into());
        }
        if pairs.is_empty() {
            return Ok(());
        }
        let pages = match &self.root {
            Some(id) => put_batch_path(store, id, None, &pairs, 0)?,
            None => write_leaves(store, pairs)?,
        };
        let root = finish_batch_root(store, pages)?;
        self.root = Some(root);
        Ok(())
    }

    pub fn persist(&mut self, _store: &mut Objects) -> Result<Option<Id>> {
        Ok(self.root.clone())
    }

    /// Returns pair count. Global ordering, exact subtree ranges, height,
    /// repeated identities/cycles and a bounded traversal budget are checked.
    pub fn audit(&mut self, store: &mut Objects) -> Result<usize> {
        fn visit(
            store: &mut Objects,
            id: &str,
            expected: Option<&Bounds>,
            seen: &mut HashSet<Id>,
            depth: usize,
        ) -> Result<usize> {
            if depth > usize::from(MAX_HEIGHT)
                || seen.len() >= MAX_AUDIT_NODES
                || !seen.insert(id.to_owned())
            {
                return Err("disk audit depth/node budget or repeated identity/cycle".into());
            }
            match read_node(store, id, expected)? {
                Node::Leaf { pairs } => Ok(pairs.len()),
                Node::Branch { height, children } => {
                    let mut count = 0usize;
                    for child in children {
                        count = count
                            .checked_add(visit(
                                store,
                                &child.id,
                                Some(&expected_child(&child, height - 1)),
                                seen,
                                depth + 1,
                            )?)
                            .ok_or("disk audit count overflow")?;
                    }
                    Ok(count)
                }
            }
        }
        match &self.root {
            None => Ok(0),
            Some(id) => visit(store, id, None, &mut HashSet::new(), 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::BTreeMap,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    fn store() -> Objects {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let path: PathBuf = std::env::temp_dir().join(format!(
            "membership-disk-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        Objects::new(path).unwrap()
    }

    #[test]
    fn model_random_updates_reopen_and_snapshot() {
        let mut store = store();
        let mut tree = DiskIndex::bulk(&mut store, vec![]).unwrap();
        let mut model = BTreeMap::new();
        let mut seed = 7u64;
        for i in 0..2400u64 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            let key = format!("k{:05}", seed % 1800);
            let value = i.to_le_bytes().to_vec();
            tree.put(&mut store, key.clone(), value.clone()).unwrap();
            model.insert(key.clone(), value.clone());
            assert_eq!(tree.get(&mut store, &key).unwrap(), Some(value));
            if i % 200 == 0 {
                let root = tree.persist(&mut store).unwrap();
                tree = DiskIndex::open(&mut store, root).unwrap();
                assert_eq!(tree.audit(&mut store).unwrap(), model.len());
            }
        }
        let mut snapshot = tree.clone();
        tree.put(&mut store, "extra".into(), vec![42]).unwrap();
        assert_eq!(snapshot.get(&mut store, "extra").unwrap(), None);
        for (key, value) in model {
            assert_eq!(tree.get(&mut store, &key).unwrap(), Some(value));
        }
        assert_eq!(tree.get(&mut store, "missing").unwrap(), None);
    }

    #[test]
    fn bulk_multiple_levels_and_byte_splits() {
        let mut store = store();
        let pairs: Vec<_> = (0..3000)
            .map(|i| (format!("{i:06}"), vec![255; 96]))
            .collect();
        let mut tree = DiskIndex::bulk(&mut store, pairs).unwrap();
        assert_eq!(tree.audit(&mut store).unwrap(), 3000);
        tree.put(&mut store, "000001".into(), vec![31]).unwrap();
        assert_eq!(tree.get(&mut store, "000001").unwrap(), Some(vec![31]));
        let pairs: Vec<_> = (0..256)
            .map(|i| {
                (
                    format!("{i:02}{}", "\u{0001}".repeat(1000)),
                    vec![255; 1024],
                )
            })
            .collect();
        let mut tree = DiskIndex::bulk(&mut store, pairs.clone()).unwrap();
        assert_eq!(tree.audit(&mut store).unwrap(), 256);
        for (key, value) in pairs {
            assert_eq!(tree.get(&mut store, &key).unwrap(), Some(value));
        }
    }

    #[test]
    fn reject_closed_schema_duplicates_order_and_trailing_data() {
        let mut store = store();
        for bytes in [
            br#"{"kind":"leaf","pairs":[["a",[]]],"extra":0}"#.as_slice(),
            br#"{"kind":"leaf","kind":"leaf","pairs":[["a",[]]]}"#.as_slice(),
            br#"{"kind":"leaf","pairs":[["a",[]]],"pairs":[["b",[]]]}"#.as_slice(),
            br#"{"kind":"leaf","pairs":[["b",[]],["a",[]]]}"#.as_slice(),
            br#"{"kind":"leaf","pairs":[["a",[]],["a",[]]]}"#.as_slice(),
            br#"{"kind":"leaf","pairs":[]}"#.as_slice(),
            br#"{"kind":"leaf","pairs":[["a",[]]]} {}"#.as_slice(),
        ] {
            let root = store.write(bytes).unwrap();
            assert!(DiskIndex::open(&mut store, Some(root)).is_err());
        }
    }

    #[test]
    fn selected_path_and_audit_reject_false_bounds() {
        let mut store = store();
        let left = write_node(
            &mut store,
            Node::Leaf {
                pairs: vec![("a".into(), vec![1])],
            },
        )
        .unwrap();
        let right = write_node(
            &mut store,
            Node::Leaf {
                pairs: vec![("z".into(), vec![2])],
            },
        )
        .unwrap();
        let mut lied = left.child;
        lied.min = "b".into();
        lied.max = "b".into();
        let root = write_node(
            &mut store,
            Node::Branch {
                height: 1,
                children: vec![lied, right.child],
            },
        )
        .unwrap()
        .child
        .id;
        let mut tree = DiskIndex::open(&mut store, Some(root)).unwrap();
        assert!(tree.get(&mut store, "a").is_err());
        assert!(tree.get(&mut store, "b").is_err());
        assert!(tree.audit(&mut store).is_err());
        // Damage outside this selected path is found by the explicit audit.
        assert_eq!(tree.get(&mut store, "z").unwrap(), Some(vec![2]));
    }

    #[test]
    fn invalid_bulk_and_bounds_do_not_replace_root() {
        let mut store = store();
        assert!(
            DiskIndex::bulk(&mut store, vec![("a".into(), vec![]), ("a".into(), vec![])]).is_err()
        );
        let mut tree = DiskIndex::bulk(&mut store, vec![("a".into(), vec![1])]).unwrap();
        let before = tree.persist(&mut store).unwrap();
        assert!(tree
            .put(&mut store, "b".into(), vec![0; MAX_VALUE_BYTES + 1])
            .is_err());
        assert_eq!(before, tree.persist(&mut store).unwrap());
    }

    // Grouped batches exercise the adapter's proposal atomicity. Publishing ROOT
    // and process locking belong to the common transaction harness.
    #[test]
    fn batch_model_reopen_and_stable_snapshot() {
        let mut store = store();
        let mut tree = DiskIndex::bulk(&mut store, vec![]).unwrap();
        let mut model = BTreeMap::new();
        let mut seed = 19u64;
        for round in 0..80u64 {
            let mut batch = BTreeMap::new();
            for j in 0..64u64 {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                batch.insert(format!("k{:05}", seed % 4096), vec![round as u8, j as u8]);
            }
            let mut old_snapshot = tree.clone();
            let old_model = model.clone();
            // Deliberately reverse input: the adapter owns sorting, while
            // duplicate keys must already have been resolved by the overlay.
            tree.put_batch(&mut store, batch.clone().into_iter().rev().collect())
                .unwrap();
            for (key, value) in batch {
                assert_eq!(
                    old_snapshot.get(&mut store, &key).unwrap(),
                    old_model.get(&key).cloned()
                );
                assert_eq!(tree.get(&mut store, &key).unwrap(), Some(value.clone()));
                model.insert(key, value);
            }
            if round % 10 == 0 {
                let root = tree.persist(&mut store).unwrap();
                tree = DiskIndex::open(&mut store, root).unwrap();
                assert_eq!(tree.audit(&mut store).unwrap(), model.len());
            }
        }
        for (key, value) in &model {
            assert_eq!(tree.get(&mut store, key).unwrap(), Some(value.clone()));
        }
        let before = tree.persist(&mut store).unwrap();
        store.metrics = crate::Metrics::default();
        tree.put_batch(
            &mut store,
            model
                .iter()
                .take(256)
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
        .unwrap();
        tree.put_batch(&mut store, vec![]).unwrap();
        assert_eq!(before, tree.persist(&mut store).unwrap());
        assert_eq!(store.metrics.nodes_written, 0);
    }

    #[test]
    fn batch_coalesces_shared_paths_against_sequential_put() {
        let initial: Vec<_> = (0..4096)
            .map(|i| (format!("k{i:05}"), vec![0; 96]))
            .collect();
        let updates: Vec<_> = (0..64).map(|i| (format!("k{i:05}"), vec![1; 96])).collect();
        let mut batch_store = store();
        let mut sequential_store = store();
        let mut batched = DiskIndex::bulk(&mut batch_store, initial.clone()).unwrap();
        let mut sequential = DiskIndex::bulk(&mut sequential_store, initial).unwrap();
        batch_store.metrics = crate::Metrics::default();
        sequential_store.metrics = crate::Metrics::default();
        batched
            .put_batch(&mut batch_store, updates.clone())
            .unwrap();
        for (key, value) in &updates {
            sequential
                .put(&mut sequential_store, key.clone(), value.clone())
                .unwrap();
        }
        assert_eq!(
            batched.persist(&mut batch_store).unwrap(),
            sequential.persist(&mut sequential_store).unwrap()
        );
        assert_eq!(batch_store.metrics.nodes_written, 3);
        assert_eq!(sequential_store.metrics.nodes_written, 64 * 3);
        assert!(batch_store.metrics.bytes_written < sequential_store.metrics.bytes_written);
        eprintln!(
            "disk batch64 vs sequential64: nodes {}/{}, bytes {}/{}",
            batch_store.metrics.nodes_written,
            sequential_store.metrics.nodes_written,
            batch_store.metrics.bytes_written,
            sequential_store.metrics.bytes_written
        );
        assert_eq!(batched.audit(&mut batch_store).unwrap(), 4096);
    }

    #[test]
    fn batch_splits_leaf_branch_and_root_then_reopens() {
        let mut store = store();
        let initial: Vec<_> = (0..2048)
            .map(|i| (format!("k{:06}", i * 2), vec![0]))
            .collect();
        let mut tree = DiskIndex::bulk(&mut store, initial).unwrap();
        let mut snapshot = tree.clone();
        // Eight insertions in each of 32 full leaves expand the full root.
        let updates: Vec<_> = (0..32)
            .flat_map(|leaf| {
                (0..8).map(move |j| (format!("k{:06}", leaf * 128 + j * 2 + 1), vec![255; 1024]))
            })
            .collect();
        tree.put_batch(&mut store, updates.clone()).unwrap();
        let root = tree.persist(&mut store).unwrap();
        tree = DiskIndex::open(&mut store, root).unwrap();
        assert_eq!(tree.audit(&mut store).unwrap(), 2304);
        assert_eq!(snapshot.audit(&mut store).unwrap(), 2048);
        for (key, value) in updates {
            assert_eq!(snapshot.get(&mut store, &key).unwrap(), None);
            assert_eq!(tree.get(&mut store, &key).unwrap(), Some(value));
        }
    }

    #[test]
    fn batch_maximal_json_expansion_reopens_and_preserves_snapshot() {
        let mut store = store();
        let initial: Vec<_> = (0..64)
            .map(|i| (format!("{i:04}{}", "\u{0001}".repeat(1020)), vec![0]))
            .collect();
        let mut tree = DiskIndex::bulk(&mut store, initial.clone()).unwrap();
        let mut snapshot = tree.clone();
        let updates: Vec<_> = initial
            .into_iter()
            .rev()
            .map(|(k, _)| (k, vec![255; 1024]))
            .collect();
        tree.put_batch(&mut store, updates.clone()).unwrap();
        let root = tree.persist(&mut store).unwrap();
        tree = DiskIndex::open(&mut store, root).unwrap();
        assert_eq!(tree.audit(&mut store).unwrap(), 64);
        for (key, value) in updates {
            assert_eq!(key.len(), MAX_KEY_BYTES);
            assert_eq!(tree.get(&mut store, &key).unwrap(), Some(value));
            assert_eq!(snapshot.get(&mut store, &key).unwrap(), Some(vec![0]));
        }
    }

    #[test]
    fn batch_rejects_invalid_duplicate_and_oversized_input_without_writes() {
        let mut store = store();
        let mut tree = DiskIndex::bulk(&mut store, vec![("a".into(), vec![1])]).unwrap();
        let root = tree.persist(&mut store).unwrap();
        store.metrics = crate::Metrics::default();
        for batch in [
            vec![("a".into(), vec![2]), ("b".into(), vec![0; 1025])],
            vec![("a".into(), vec![2]), ("a".into(), vec![3])],
            (0..257).map(|i| (format!("k{i}"), vec![1])).collect(),
        ] {
            assert!(tree.put_batch(&mut store, batch).is_err());
            assert_eq!(tree.persist(&mut store).unwrap(), root);
            assert_eq!(tree.get(&mut store, "a").unwrap(), Some(vec![1]));
        }
        assert_eq!(store.metrics.nodes_written, 0);
    }

    #[test]
    fn batch_write_failure_keeps_original_root_with_unreachable_proposal() {
        let mut store = store();
        let initial: Vec<_> = (0..128).map(|i| (format!("k{i:05}"), vec![0])).collect();
        let mut tree = DiskIndex::bulk(&mut store, initial.clone()).unwrap();
        let root = tree.persist(&mut store).unwrap();
        let mut right = initial[64..].to_vec();
        right[0].1 = vec![9];
        let bytes = serde_json::to_vec(&Node::Leaf { pairs: right }).unwrap();
        let mut frame = crate::MAGIC.to_vec();
        frame.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        frame.extend_from_slice(&bytes);
        let failed_id = crate::hash(&frame);
        let shard = store.base.join("objects").join(&failed_id[..2]);
        std::fs::create_dir_all(&shard).unwrap();
        // create_new for the second changed leaf must fail, after the first leaf
        // has installed an object. No environment-global fault races are needed.
        let collision = shard.join(format!("stage-{}-{}", std::process::id(), store.serial + 2));
        std::fs::write(&collision, b"preexisting stage").unwrap();
        store.metrics = crate::Metrics::default();
        assert!(tree
            .put_batch(
                &mut store,
                vec![("k00000".into(), vec![8]), ("k00064".into(), vec![9])]
            )
            .is_err());
        assert_eq!(store.metrics.nodes_written, 1);
        assert_eq!(tree.persist(&mut store).unwrap(), root);
        let mut reopened = DiskIndex::open(&mut store, root).unwrap();
        assert_eq!(reopened.audit(&mut store).unwrap(), 128);
        assert_eq!(reopened.get(&mut store, "k00000").unwrap(), Some(vec![0]));
        assert_eq!(reopened.get(&mut store, "k00064").unwrap(), Some(vec![0]));
    }
}
