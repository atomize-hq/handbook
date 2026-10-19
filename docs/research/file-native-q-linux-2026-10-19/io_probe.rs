use file_native_incremental::io::{self,Metrics};
use std::{path::PathBuf,fs,process::Command};
fn main(){
 let a:Vec<String>=std::env::args().collect();let root=PathBuf::from(&a[1]);fs::create_dir_all(&root).unwrap();
 if a.len()>2{let mut m=Metrics::default();let stage=root.join("interrupted-stage");io::write_new(&stage,b"immutable",&mut m).unwrap();io::install_original(&stage,&root.join("interrupted-winner"),&io::digest(b"immutable"),&mut m).unwrap();return;}
 let mut jobs=vec![];
 for n in 0..8{let r=root.clone();jobs.push(std::thread::spawn(move||{let mut m=Metrics::default();let s=r.join(format!("race-{n}"));io::write_new(&s,b"immutable",&mut m).unwrap();io::install_original(&s,&r.join("winner"),&io::digest(b"immutable"),&mut m).unwrap();m.directory_syncs}));}
 let mut syncs=0;for j in jobs{syncs+=j.join().unwrap();}assert_eq!(fs::read(root.join("winner")).unwrap(),b"immutable");
 let mut m=Metrics::default();let stage=root.join("different");io::write_new(&stage,b"different",&mut m).unwrap();assert!(io::install_original(&stage,&root.join("winner"),&io::digest(b"different"),&mut m).is_err());assert!(stage.exists());
 fs::write(root.join("winner"),b"corrupt").unwrap();assert!(io::install_original(&stage,&root.join("winner"),&io::digest(b"immutable"),&mut m).is_err());assert_eq!(fs::read(root.join("winner")).unwrap(),b"corrupt");
 io::atomic(&root.join("selector"),b"one",&mut m).unwrap();io::atomic(&root.join("selector"),b"two",&mut m).unwrap();assert_eq!(fs::read(root.join("selector")).unwrap(),b"two");
 let status=Command::new(std::env::current_exe().unwrap()).arg(&root).arg("child").env("EXP_EXIT","immutable-name-installed").status().unwrap();assert_eq!(status.code(),Some(86));
 let before=fs::metadata(root.join("interrupted-winner")).unwrap();use std::os::unix::fs::MetadataExt;let inode=before.ino();io::install_original(&root.join("interrupted-stage"),&root.join("interrupted-winner"),&io::digest(b"immutable"),&mut m).unwrap();assert_eq!(inode,fs::metadata(root.join("interrupted-winner")).unwrap().ino());
 assert!(syncs+m.directory_syncs>0);println!("{{\"same_content_writers\":8,\"different_corrupt_winner_refused\":true,\"replacement\":true,\"directory_syncs\":{},\"installed_name_exit86_inode_retry\":true}}",syncs+m.directory_syncs);
}
