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
const MAX_VALUE_BYTES: usize = 8192;
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
    let node: Node = decode_node(&bytes)?;
    let actual = bounds(&node)?;
    if expected.is_some_and(|expected| expected != &actual) {
        return Err("disk selected child does not match parent bounds/height".into());
    }
    Ok(node)
}

fn write_node(store: &mut Objects, node: Node) -> Result<Written> {
    let b = bounds(&node)?;
    let bytes = encode_node(&node)?;
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
    Ok(encode_node(&Node::Leaf {
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
            && encode_node(&Node::Branch {
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


impl DiskIndex {
    pub fn entries(&mut self, store: &mut Objects) -> Result<Vec<(String,Vec<u8>)>> {
        fn visit(store:&mut Objects,id:&str,expected:Option<&Bounds>,out:&mut Vec<(String,Vec<u8>)>,seen:&mut HashSet<Id>)->Result<()> {
            if !seen.insert(id.to_owned()) || seen.len()>MAX_AUDIT_NODES {return Err("integrity:cow-cycle-or-cap".into());}
            match read_node(store,id,expected)? {
                Node::Leaf{pairs}=>out.extend(pairs),
                Node::Branch{height,children}=>for c in children {visit(store,&c.id,Some(&expected_child(&c,height-1)),out,seen)?;},
            } Ok(())
        }
        let mut out=vec![];if let Some(id)=&self.root {visit(store,id,None,&mut out,&mut HashSet::new())?;}Ok(out)
    }
}


// C1 is private: magic/version/tag, bounded counts, length-prefixed UTF-8 keys,
// raw value bytes and raw 32-byte child hashes. Logical capacities are unchanged.
fn encode_node(node: &Node) -> Result<Vec<u8>> {
    if std::env::var("EXP_CODEC").ok().as_deref()!=Some("compact") {
        return Ok(serde_json::to_vec(node)?);
    }
    let mut b=b"FNC1".to_vec();
    fn key(b:&mut Vec<u8>,s:&str)->Result<()> {
        if s.len()>MAX_KEY_BYTES {return Err("codec:key-bound".into());}
        b.extend((s.len() as u16).to_le_bytes());b.extend(s.as_bytes());Ok(())
    }
    match node {
        Node::Leaf{pairs}=>{
            b.push(0);b.extend((pairs.len() as u16).to_le_bytes());
            for (k,v) in pairs {key(&mut b,k)?;b.extend((v.len() as u32).to_le_bytes());b.extend(v);}
        },
        Node::Branch{height,children}=>{
            b.push(1);b.push(*height);b.extend((children.len() as u16).to_le_bytes());
            for c in children {
                if !valid_id(&c.id){return Err("codec:child-id".into());}
                for i in (0..64).step_by(2){b.push(u8::from_str_radix(&c.id[i..i+2],16)?);}
                key(&mut b,&c.min)?;key(&mut b,&c.max)?;
            }
        }
    }
    if b.len()>MAX_OBJECT_BYTES{return Err("codec:object-bound".into());}Ok(b)
}
fn decode_node(bytes:&[u8])->Result<Node> {
    if bytes.len()>MAX_OBJECT_BYTES{return Err("codec:object-bound".into());}
    if bytes.first()==Some(&b'{') {return Ok(serde_json::from_slice(bytes)?);}
    struct Cursor<'a>{b:&'a [u8],p:usize}
    impl Cursor<'_>{
        fn take(&mut self,n:usize)->Result<&[u8]>{
            let end=self.p.checked_add(n).ok_or("codec:length-overflow")?;
            if end>self.b.len(){return Err("codec:truncated".into());}
            let v=&self.b[self.p..end];self.p=end;Ok(v)
        }
        fn u16(&mut self)->Result<usize>{Ok(u16::from_le_bytes(self.take(2)?.try_into()?) as usize)}
        fn key(&mut self)->Result<String>{
            let n=self.u16()?;if n>MAX_KEY_BYTES{return Err("codec:key-bound".into());}
            Ok(std::str::from_utf8(self.take(n)?)?.to_owned())
        }
    }
    let mut c=Cursor{b:bytes,p:0};
    if c.take(4)?!=b"FNC1"{return Err("codec:unsupported-version".into());}
    let tag=c.take(1)?[0];
    let node=match tag{
        0=>{
            let n=c.u16()?;if n==0||n>LEAF_CAPACITY{return Err("codec:leaf-count".into());}
            let mut pairs=Vec::with_capacity(n);
            for _ in 0..n {
                let k=c.key()?;let n=u32::from_le_bytes(c.take(4)?.try_into()?) as usize;
                if n>MAX_VALUE_BYTES{return Err("codec:value-bound".into());}
                pairs.push((k,c.take(n)?.to_vec()));
            }Node::Leaf{pairs}
        },
        1=>{
            let height=c.take(1)?[0];let n=c.u16()?;
            if n<2||n>FANOUT{return Err("codec:branch-count".into());}
            let mut children=Vec::with_capacity(n);
            for _ in 0..n{
                let id=c.take(32)?.iter().map(|b|format!("{b:02x}")).collect();
                children.push(Child{id,min:c.key()?,max:c.key()?});
            }Node::Branch{height,children}
        },
        _=>return Err("codec:tag".into())
    };
    if c.p!=bytes.len(){return Err("codec:trailing".into());}bounds(&node)?;Ok(node)
}
impl DiskIndex {
    pub fn reachable(&mut self,store:&mut Objects)->Result<Vec<Id>>{
        fn visit(store:&mut Objects,id:&str,expected:Option<&Bounds>,seen:&mut HashSet<Id>,out:&mut Vec<Id>)->Result<()>{
            if !seen.insert(id.to_owned()){return Err("integrity:repeated-tree-page".into());}
            if seen.len()>MAX_AUDIT_NODES{return Err("integrity:audit-cap".into());}
            out.push(id.to_owned());
            match read_node(store,id,expected)?{
                Node::Branch{height,children}=>for c in children{visit(store,&c.id,Some(&expected_child(&c,height-1)),seen,out)?;},
                Node::Leaf{pairs}=>for (_,b) in pairs{
                    if b.first()==Some(&b'@'){
                        let id=std::str::from_utf8(&b[1..])?;
                        if !valid_id(id){return Err("integrity:indirection".into());}
                        store.read(id)?;out.push(id.into());
                    }
                }
            }Ok(())
        }
        let mut out=vec![];if let Some(id)=&self.root{visit(store,id,None,&mut HashSet::new(),&mut out)?;}Ok(out)
    }
    pub fn range(&mut self,store:&mut Objects,lo:&str,hi:&str)->Result<Vec<(String,Vec<u8>)>>{
        fn visit(store:&mut Objects,id:&str,expected:Option<&Bounds>,lo:&str,hi:&str,out:&mut Vec<(String,Vec<u8>)>,depth:usize)->Result<()>{
            if depth>MAX_HEIGHT as usize{return Err("integrity:range-depth".into());}
            match read_node(store,id,expected)?{
                Node::Leaf{pairs}=>out.extend(pairs.into_iter().filter(|(k,_)|k.as_str()>=lo&&k.as_str()<hi)),
                Node::Branch{height,children}=>for c in children{
                    if c.max.as_str()>=lo&&c.min.as_str()<hi{visit(store,&c.id,Some(&expected_child(&c,height-1)),lo,hi,out,depth+1)?;}
                }
            }Ok(())
        }
        let mut out=vec![];if let Some(id)=&self.root{visit(store,id,None,lo,hi,&mut out,0)?;}Ok(out)
    }
}
#[cfg(test)]
mod codec_tests {
    use super::*;
    #[test]
    fn compact_refuses_damage_before_allocating(){
        std::env::set_var("EXP_CODEC","compact");
        let node=Node::Leaf{pairs:vec![("a".into(),vec![0,1,255])]};
        let b=encode_node(&node).unwrap();assert!(b.len()<32);
        for n in 0..b.len(){assert!(decode_node(&b[..n]).is_err());}
        let mut bad=b.clone();bad[3]=b'9';assert!(decode_node(&bad).is_err());
        let mut bad=b.clone();bad.push(0);assert!(decode_node(&bad).is_err());
        let mut bad=b.clone();bad[5]=255;bad[6]=255;assert!(decode_node(&bad).is_err());
        let mut bad=b.clone();bad[10..14].copy_from_slice(&u32::MAX.to_le_bytes());assert!(decode_node(&bad).is_err());
        assert!(decode_node(&vec![0;MAX_OBJECT_BYTES+1]).is_err());
        assert_eq!(encode_node(&decode_node(&b).unwrap()).unwrap(),b);
    }
}
