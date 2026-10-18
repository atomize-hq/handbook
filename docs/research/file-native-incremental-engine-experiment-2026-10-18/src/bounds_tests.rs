use crate::{io::{self,Metrics},objects::{self,Objects},disk::DiskIndex,graph::{self,GraphRoot}};
#[test]
fn bounded_file_and_indirection_routes_refuse(){
    let root=std::env::temp_dir().join(io::unique("i1-bounds"));
    std::fs::create_dir(&root).unwrap();
    let path=root.join("oversized");std::fs::File::create(&path).unwrap().set_len(1025).unwrap();
    let mut m=Metrics::default();
    assert!(io::read_bounded(&path,1024,&mut m).is_err());assert_eq!(m.bytes_read,0);
    let mut o=Objects{base:root.clone(),metrics:&mut m};
    let mut idx=DiskIndex::bulk(&mut o,vec![("D/x".into(),b"@invalid".to_vec())]).unwrap();
    let r=idx.persist(&mut o).unwrap();
    assert!(objects::point::<graph::Header>(&mut o,&r,"D/x").is_err());
    let g=GraphRoot{version:1,consumer:"project".into(),budget:1000,cost:0,count:1,tree:r.clone(),fallback:false};
    assert!(graph::all(&mut o,&g).is_err());
    assert!(graph::audit(&mut o,&g).is_err());
    assert!(objects::reachable(&mut o,&r).is_err());
}
