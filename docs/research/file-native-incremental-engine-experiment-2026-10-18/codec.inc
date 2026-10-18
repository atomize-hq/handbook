
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
