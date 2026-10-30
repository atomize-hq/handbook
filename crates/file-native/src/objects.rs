use crate::{backend::Storage, digest, Id, Result, MAX_OBJECT, MAX_TRANSACTION_BYTES};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) fn path(id: &str) -> Result<String> {
    if id.len() != 64
        || !id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("object identity".into());
    }
    Ok(format!("objects/{}/{}/{}", &id[..2], &id[2..4], id))
}
pub(crate) struct Objects<'a> {
    io: &'a dyn Storage,
    token: String,
    pub pending: BTreeMap<Id, Vec<u8>>,
    pub retired: BTreeSet<Id>,
    pub bytes: usize,
    pub reads: u64,
    next: u64,
}
impl<'a> Objects<'a> {
    pub fn new(io: &'a dyn Storage, token: &str) -> Self {
        Self {
            io,
            token: token.into(),
            pending: BTreeMap::new(),
            retired: BTreeSet::new(),
            bytes: 0,
            reads: 0,
            next: 0,
        }
    }
    pub fn read(&mut self, id: &str) -> Result<Vec<u8>> {
        self.reads += 1;
        let b = match self.pending.get(id) {
            Some(b) => b.clone(),
            None => self
                .io
                .read(&path(id)?, MAX_OBJECT + 76)?
                .ok_or("required immutable object missing")?,
        };
        if digest(&b) != id || b.len() < 76 || &b[..4] != b"FNO2" {
            return Err("immutable object proof".into());
        }
        Ok(b[76..].to_vec())
    }
    pub fn write(&mut self, b: &[u8]) -> Result<Id> {
        if b.len() > MAX_OBJECT || self.token.len() != 64 {
            return Err("object size/write context".into());
        }
        let ordinal = self.next;
        self.next = self
            .next
            .checked_add(1)
            .ok_or("object ownership ordinal overflow")?;
        let mut framed = b"FNO2".to_vec();
        framed.extend(self.token.as_bytes());
        framed.extend(ordinal.to_be_bytes());
        framed.extend(b);
        let id = digest(&framed);
        if !self.pending.contains_key(&id) {
            self.bytes = self
                .bytes
                .checked_add(framed.len())
                .ok_or("allocation overflow")?;
            if self.bytes > MAX_TRANSACTION_BYTES || self.pending.len() >= 4096 {
                return Err("transaction allocation bound".into());
            }
            self.pending.insert(id.clone(), framed);
        }
        Ok(id)
    }
    pub fn retire(&mut self, id: &str) {
        if let Some(b) = self.pending.remove(id) {
            self.bytes -= b.len();
        } else {
            self.retired.insert(id.into());
        }
    }
}
