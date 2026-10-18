"""One-shot construction of the copied control; never mutates the baseline."""
from pathlib import Path
p=Path(__file__).parent
io=p/'control/src/io.rs'
s=io.read_text()
a=s.index('pub fn read(path:')
b=s.index('pub fn sync_file',a)
s=s[:a]+'''pub fn read(path: &Path, m: &mut Metrics) -> Result<Vec<u8>> {
    read_bounded(path, 128*1024*1024, m)
}
pub fn read_bounded(path: &Path, limit: usize, m: &mut Metrics) -> Result<Vec<u8>> {
    let t=Instant::now();
    let f=File::open(path).map_err(|e|format!("integrity-read:{}:{e}",path.display()))?;
    if f.metadata()?.len()>limit as u64 {return Err("integrity:file-length-bound".into());}
    let mut b=Vec::new();
    f.take(limit as u64+1).read_to_end(&mut b)?;
    if b.len()>limit {return Err("integrity:file-length-bound".into());}
    m.opens+=1; m.bytes_read+=b.len() as u64; m.elapsed("read",t); Ok(b)
}
'''+s[b:]
io.write_text(s)
disk=p/'control/src/disk.rs'
s=disk.read_text()
s=s.replace('serde_json::from_slice(&bytes)?','decode_node(&bytes)?')
s=s.replace('serde_json::to_vec(&node)?','encode_node(&node)?')
s=s.replace('serde_json::to_vec(&Node::Leaf {','encode_node(&Node::Leaf {')
s=s.replace('serde_json::to_vec(&Node::Branch {','encode_node(&Node::Branch {')
s+='\n'+(p/'codec.inc').read_text()
disk.write_text(s)
back=p/'control/src/backend.rs'
s=back.read_text()
s=s.replace('let b=io::checked(&self.base.join("objects").join(&id[..2]).join(id),id,self.metrics)?;',
'''let b=io::read_bounded(&self.base.join("objects").join(&id[..2]).join(id),crate::MAX_OBJECT,self.metrics)?;
        if io::digest(&b)!=id{return Err("integrity:object-digest".into());}''')
a=s.index('    fn visit(o:',s.index('pub fn reachable'))
s=s[:a]+'''    idx.reachable(&mut o)
}
'''
back.write_text(s)
