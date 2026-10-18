use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs::{self, File, OpenOptions}, io::{Read, Write}, path::Path, sync::atomic::{AtomicU64, Ordering}, time::{Instant, SystemTime, UNIX_EPOCH}};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
static SEQ: AtomicU64 = AtomicU64::new(0);
#[derive(Default, Serialize)]
pub struct Metrics {
    pub ns: BTreeMap<String,u128>,
    pub bytes_read: u64, pub bytes_written: u64, pub opens: u64,
    pub file_syncs: u64, pub directory_syncs: u64, pub unsupported_directory_syncs: u64,
    pub copies: u64, pub replacements: u64, pub created_directories: u64,
    pub bytes_metadata_read:u64, pub bytes_metadata_written:u64,
}
impl Metrics {
    pub fn elapsed(&mut self, name: &str, start: Instant) { *self.ns.entry(name.into()).or_default() += start.elapsed().as_nanos(); }
}
pub fn digest(bytes: &[u8]) -> String { format!("{:x}",Sha256::digest(bytes)) }
pub fn unique(prefix: &str) -> String { format!("{}-{}-{}-{}",prefix,std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),SEQ.fetch_add(1,Ordering::Relaxed)) }
pub fn checkpoint(name: &str) -> Result<()> {
    if std::env::var("EXP_TRACE").is_ok() { eprintln!("checkpoint:{name}"); }
    if let Ok(n)=std::env::var("EXP_NTH") {
        static COUNTS:std::sync::Mutex<BTreeMap<String,u64>>=std::sync::Mutex::new(BTreeMap::new());
        let mut counts=COUNTS.lock().unwrap();let count=counts.entry(name.into()).or_default();*count+=1;
        if *count!=n.parse::<u64>()? {return Ok(());}
    }
    if std::env::var("EXP_BARRIER").ok().as_deref()==Some(name) {
        if let Ok(p)=std::env::var("EXP_READY") { fs::write(p,name)?; }
        let release=std::env::var("EXP_RELEASE")?;
        let start=Instant::now();
        while !Path::new(&release).exists() {
            if start.elapsed().as_secs()>60 { return Err("barrier-timeout".into()); }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    if std::env::var("EXP_EXIT").ok().as_deref()==Some(name) { std::process::exit(86); }
    if std::env::var("EXP_ERROR").ok().as_deref()==Some(name) { return Err(format!("injected-io:{name}").into()); }
    Ok(())
}
pub fn mkdir(path: &Path, m: &mut Metrics) -> Result<()> {
    if path.is_dir() { if let Some(p)=path.parent(){if p!=path {sync_dir(p,m)?;}} return Ok(()); }
    if let Some(p)=path.parent() { mkdir(p,m)?; }
    let create=Instant::now();let created=match fs::create_dir(path){Ok(())=>true,Err(e) if e.kind()==std::io::ErrorKind::AlreadyExists&&path.is_dir()=>false,Err(e)=>return Err(e.into())};m.elapsed("directory-create",create);
    if created {m.created_directories+=1;checkpoint("directory-create")?;}
    if path.to_string_lossy().contains("generation-stage"){checkpoint("generation-directory-create")?;}
    if path.to_string_lossy().contains("view"){checkpoint("view-directory-create")?;}
    if let Some(p)=path.parent() { sync_dir(p,m)?; }
    Ok(())
}
pub fn read(path: &Path, m: &mut Metrics) -> Result<Vec<u8>> {
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
pub fn sync_file(path: &Path, m: &mut Metrics) -> Result<()> {
    let t=Instant::now(); OpenOptions::new().read(true).write(true).open(path)?.sync_all()?;
    m.opens+=1; m.file_syncs+=1; m.elapsed("sync",t); Ok(())
}
pub fn sync_dir(path: &Path, m: &mut Metrics) -> Result<()> {
    let t=Instant::now();
    #[cfg(unix)] { File::open(path)?.sync_all()?; m.directory_syncs+=1; }
    #[cfg(windows)] {
        // NTFS directory handles opened with backup semantics do not supply an
        // unprivileged portable fsync equivalent. Never count this as success.
        let _=path; m.unsupported_directory_syncs+=1;
    }
    m.elapsed("sync",t); Ok(())
}
pub fn write_new(path: &Path, bytes: &[u8], m: &mut Metrics) -> Result<()> {
    mkdir(path.parent().ok_or("no-parent")?,m)?;
    let t=Instant::now(); let mut f=OpenOptions::new().create_new(true).write(true).open(path)?;
    m.opens+=1; f.write_all(bytes)?; m.bytes_written+=bytes.len() as u64;
    checkpoint("file-write")?;
    let role=if path.file_name().unwrap().to_string_lossy().starts_with("selector-stage"){"selector"}else if path.parent().unwrap().ends_with("publications"){"publication"}else if path.file_name().unwrap()=="state.json"{"state"}else if path.file_name().unwrap()=="MANIFEST.json"{"manifest"}else if path.parent().unwrap().ends_with("receipts"){"receipt"}else{"other"};
    checkpoint(&format!("{role}-file-write"))?;m.elapsed("write",t);
    let t=Instant::now(); f.sync_all()?; m.file_syncs+=1; m.elapsed("sync",t); checkpoint("file-sync")?;checkpoint(&format!("{role}-file-sync"))?;
    sync_dir(path.parent().unwrap(),m)?; Ok(())
}
pub fn replace(from: &Path, to: &Path, m: &mut Metrics) -> Result<()> {
    let t=Instant::now();
    #[cfg(windows)] {
        use std::os::windows::ffi::OsStrExt;
        let a:Vec<u16>=from.as_os_str().encode_wide().chain(Some(0)).collect();
        let b:Vec<u16>=to.as_os_str().encode_wide().chain(Some(0)).collect();
        let ok=unsafe { windows_sys::Win32::Storage::FileSystem::MoveFileExW(a.as_ptr(),b.as_ptr(),0x1|0x8) };
        if ok==0 { return Err(std::io::Error::last_os_error().into()); }
    }
    #[cfg(unix)] fs::rename(from,to)?;
    m.replacements+=1; m.elapsed("replace",t); Ok(())
}
pub fn install_original(stage:&Path,dest:&Path,expected:&str,m:&mut Metrics)->Result<()> {
    // Windows no-replace install can lose a same-content race. Verify the winner
    // and retain our owned stage. Bounded retries cover transient sharing/rename
    // refusal; no selected mutation or arbitrary external effect is replayed.
    #[cfg(windows)] {
        use std::os::windows::ffi::OsStrExt;
        let a:Vec<u16>=stage.as_os_str().encode_wide().chain(Some(0)).collect();
        let b:Vec<u16>=dest.as_os_str().encode_wide().chain(Some(0)).collect();
        for attempt in 0..5 {
            let ok=unsafe{windows_sys::Win32::Storage::FileSystem::MoveFileExW(a.as_ptr(),b.as_ptr(),0x8)};
            if ok!=0 {m.replacements+=1;return Ok(());}
            let error=std::io::Error::last_os_error();
            if dest.exists(){return checked(dest,expected,m).map(|_|());}
            if attempt==4{return Err(format!("refused:original-install:{error}").into());}
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    #[cfg(unix)] {
        // link(2) atomically installs a new name without replacing a winner.
        // Source is an owned, fully written and synced file on the same fs.
        // Preserve the stage on refusal; never repair a corrupt installed file.
        match fs::hard_link(stage,dest) {
            Ok(()) => { m.replacements+=1; checkpoint("immutable-name-installed")?; sync_dir(dest.parent().unwrap(),m)?; fs::remove_file(stage)?; sync_dir(stage.parent().unwrap(),m)?; },
            Err(e) if e.kind()==std::io::ErrorKind::AlreadyExists => {checked(dest,expected,m)?;sync_dir(dest.parent().unwrap(),m)?;},
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
pub fn atomic(path: &Path, bytes: &[u8], m: &mut Metrics) -> Result<()> {
    let stage=path.with_file_name(unique("stage")); write_new(&stage,bytes,m)?;
    replace(&stage,path,m)?; sync_dir(path.parent().unwrap(),m)
}
pub fn copy(source: &Path, dest: &Path, m: &mut Metrics) -> Result<()> {
    mkdir(dest.parent().unwrap(),m)?; let t=Instant::now();
    let mut src=File::open(source)?; let mut dst=OpenOptions::new().write(true).create_new(true).open(dest)?;
    m.opens+=2; let mut buf=[0u8;65536];
    loop { let n=src.read(&mut buf)?; if n==0 {break;} dst.write_all(&buf[..n])?; m.bytes_read+=n as u64; m.bytes_written+=n as u64; }
    m.copies+=1; m.elapsed("copy",t); checkpoint("copy-write")?;
    let role=if dest.file_name().unwrap().to_string_lossy().starts_with("view-stage"){"view"}else if dest.to_string_lossy().contains("generation-stage"){"generation"}else{"packet"};
    checkpoint(&format!("{role}-copy-write"))?;
    let t=Instant::now(); dst.sync_all()?; m.file_syncs+=1; m.elapsed("sync",t);
    checkpoint("copy-sync")?;checkpoint(&format!("{role}-copy-sync"))?; sync_dir(dest.parent().unwrap(),m)?; Ok(())
}
pub fn checked(path: &Path, expected: &str, m: &mut Metrics) -> Result<Vec<u8>> {
    let b=read(path,m)?; if digest(&b)!=expected { return Err(format!("integrity-digest:{}",path.display()).into()); } Ok(b)
}
pub fn json<T:Serialize>(value: &T) -> Result<Vec<u8>> { Ok(serde_json::to_vec_pretty(value)?) }
pub fn peak_memory() -> Option<u64> {
    #[cfg(windows)] unsafe {
        use windows_sys::Win32::System::{ProcessStatus::{GetProcessMemoryInfo,PROCESS_MEMORY_COUNTERS},Threading::GetCurrentProcess};
        let mut p:PROCESS_MEMORY_COUNTERS=std::mem::zeroed(); p.cb=std::mem::size_of_val(&p) as u32;
        if GetProcessMemoryInfo(GetCurrentProcess(),&mut p,p.cb)!=0 { return Some(p.PeakWorkingSetSize as u64); }
    }
    #[cfg(unix)] { if let Ok(s)=fs::read_to_string("/proc/self/status") { return s.lines().find(|l|l.starts_with("VmHWM:")).and_then(|l|l.split_whitespace().nth(1)).and_then(|s|s.parse::<u64>().ok()).map(|n|n*1024); } }
    None
}
