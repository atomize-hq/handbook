//! Non-production experiment. See ../contract.md for the tested threat bounds.
mod chunk;
mod disk;

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

pub type Id = String;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const MAX_OBJECT: usize = 128 * 1024;
const MAGIC: &[u8] = b"HMP1";

#[derive(Default, Serialize)]
pub struct Metrics {
    reads: u64,
    writes: u64,
    bytes_read: u64,
    bytes_written: u64,
    opens: u64,
    stats: u64,
    lists: u64,
    listed_entries: u64,
    guards: u64,
    nodes_loaded: u64,
    nodes_written: u64,
    lock_wait_ns: u64,
    lock_hold_ns: u64,
    retries: u64,
    syncs: u64,
    replacements: u64,
}

fn hash(bytes: &[u8]) -> Id {
    format!("{:x}", Sha256::digest(bytes))
}
fn valid_id(id: &str) -> bool {
    id.len() == 64
        && id
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn checkpoint(name: &str) -> Result<()> {
    if std::env::var("PROTO_FAULT").ok().as_deref() == Some(name) {
        std::process::exit(86); // process death; OS cache survives
    }
    if std::env::var("PROTO_FAIL").ok().as_deref() == Some(name) {
        return Err(format!("injected I/O failure at {name}; publication may be uncertain").into());
    }
    Ok(())
}

fn plain(path: &Path) -> Result<fs::Metadata> {
    let m = fs::symlink_metadata(path)?;
    if m.file_type().is_symlink() {
        return Err("symlink refused".into());
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if m.file_attributes() & 0x400 != 0 {
            return Err("reparse point refused".into());
        }
    }
    Ok(m)
}

fn directory(path: &Path) -> Result<File> {
    if !plain(path)?.is_dir() {
        return Err("directory required".into());
    }
    let mut opts = OpenOptions::new();
    opts.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        opts.custom_flags(windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS);
    }
    Ok(opts.open(path)?)
}

fn identity(file: &File) -> Result<String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let m = file.metadata()?;
        Ok(format!("{}:{}", m.dev(), m.ino()))
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::{
            GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
        };
        let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
        if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut info) } == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(format!(
            "{}:{}:{}",
            info.dwVolumeSerialNumber, info.nFileIndexHigh, info.nFileIndexLow
        ))
    }
}

// Linux: fsync on the containing directory, separately from the file.
// https://man7.org/linux/man-pages/man2/fsync.2.html
// Windows: no directory-fsync claim. Each rename uses MoveFileExW WRITE_THROUGH;
// file handles use FlushFileBuffers via sync_all. This experiment qualifies process
// interruption only; directory persistence after power loss is still unqualified.
fn sync_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        directory(path)?.sync_all()?;
    }
    #[cfg(windows)]
    {
        let _ = directory(path)?;
    }
    Ok(())
}

fn replace(from: &Path, to: &Path, overwrite: bool) -> Result<()> {
    #[cfg(unix)]
    {
        // Cooperating writers hold the same native lock. rename itself is not CAS.
        if !overwrite && to.exists() {
            return Err("destination already exists".into());
        }
        fs::rename(from, to)?;
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::Storage::FileSystem::{
            MoveFileExW, MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH,
        };
        let a: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
        let b: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();
        let flags = MOVEFILE_WRITE_THROUGH
            | if overwrite {
                MOVEFILE_REPLACE_EXISTING
            } else {
                0
            };
        if unsafe { MoveFileExW(a.as_ptr(), b.as_ptr(), flags) } == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
    }
    Ok(())
}

pub struct Objects {
    pub base: PathBuf,
    pub metrics: Metrics,
    instrument: bool,
    guard_list: bool,
    dirty_dirs: BTreeSet<PathBuf>,
    serial: u64,
}

impl Objects {
    pub fn new(base: PathBuf) -> Result<Self> {
        fs::create_dir_all(&base)?;
        plain(&base)?;
        if !base.join("OWNER").exists() {
            fs::create_dir_all(base.join("objects"))?;
        }
        plain(&base.join("objects"))?;
        Ok(Self {
            base,
            metrics: Metrics::default(),
            instrument: true,
            guard_list: false,
            dirty_dirs: BTreeSet::new(),
            serial: 0,
        })
    }
    fn guard(&mut self, path: &Path) -> Result<()> {
        if self.instrument {
            self.metrics.guards += 1;
            self.metrics.stats += 1;
        }
        plain(path)?;
        if self.guard_list {
            if self.instrument {
                self.metrics.lists += 1;
            }
            for e in fs::read_dir(self.base.join("siblings"))? {
                e?;
                if self.instrument {
                    self.metrics.listed_entries += 1;
                }
            }
        }
        Ok(())
    }
    fn path(&self, id: &str) -> Result<PathBuf> {
        if !valid_id(id) {
            return Err("invalid durable object ID".into());
        }
        Ok(self.base.join("objects").join(&id[..2]).join(id))
    }
    pub fn read(&mut self, id: &str) -> Result<Vec<u8>> {
        let p = self.path(id)?;
        self.guard(&self.base.join("objects"))?;
        self.guard(p.parent().unwrap())?;
        self.guard(&p)?;
        if self.instrument {
            self.metrics.opens += 1;
        }
        let mut f = File::open(&p)?;
        let size = f.metadata()?.len();
        if self.instrument {
            self.metrics.stats += 1;
        }
        if size < 8 || size > (MAX_OBJECT + 8) as u64 {
            return Err("object frame length refused".into());
        }
        let mut bytes = Vec::new();
        (&mut f)
            .take((MAX_OBJECT + 9) as u64)
            .read_to_end(&mut bytes)?;
        if bytes.len() != size as usize || &bytes[..4] != MAGIC {
            return Err("torn object frame".into());
        }
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        if n + 8 != bytes.len() || hash(&bytes) != id {
            return Err("object checksum/length mismatch".into());
        }
        if self.instrument {
            self.metrics.reads += 1;
            self.metrics.nodes_loaded += 1;
            self.metrics.bytes_read += bytes.len() as u64;
        }
        Ok(bytes[8..].to_vec())
    }
    pub fn write(&mut self, bytes: &[u8]) -> Result<Id> {
        if bytes.len() > MAX_OBJECT {
            return Err("object encoding too large".into());
        }
        let mut framed = MAGIC.to_vec();
        framed.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        framed.extend_from_slice(bytes);
        let id = hash(&framed);
        let p = self.path(&id)?;
        let parent = p.parent().unwrap().to_path_buf();
        fs::create_dir_all(&parent)?;
        self.guard(&parent)?;
        self.dirty_dirs.insert(self.base.join("objects"));
        if p.exists() {
            if self.read(&id)? != bytes {
                return Err("existing object mismatch".into());
            }
            return Ok(id);
        }
        self.serial += 1;
        let stage = parent.join(format!("stage-{}-{}", std::process::id(), self.serial));
        // Stale staging names are retained after interrupted proposals. Never reused.
        let mut f = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&stage)?;
        if self.instrument {
            self.metrics.opens += 1;
        }
        if std::env::var("PROTO_FAIL").ok().as_deref() == Some("short_write") {
            f.write_all(&framed[..framed.len() / 2])?;
            return Err("injected short write (incomplete unselected staging object)".into());
        }
        f.write_all(&framed)?;
        checkpoint("object_write")?;
        f.sync_all()?;
        checkpoint("object_sync")?;
        drop(f);
        replace(&stage, &p, false)?;
        self.dirty_dirs.insert(parent);
        if self.instrument {
            self.metrics.writes += 1;
            self.metrics.nodes_written += 1;
            self.metrics.bytes_written += framed.len() as u64;
            self.metrics.syncs += 1;
            self.metrics.replacements += 1;
        }
        Ok(id)
    }
    fn flush_names(&mut self) -> Result<()> {
        for p in &self.dirty_dirs {
            sync_directory(p)?;
        }
        self.dirty_dirs.clear();
        checkpoint("objects_dir_sync")?;
        Ok(())
    }
    fn read_record(&mut self, path: &Path, limit: usize) -> Result<Vec<u8>> {
        self.guard(path)?;
        if self.instrument {
            self.metrics.opens += 1;
            self.metrics.stats += 1;
        }
        let mut f = File::open(path)?;
        let len = f.metadata()?.len();
        if len > limit as u64 {
            return Err("metadata limit".into());
        }
        let mut bytes = Vec::new();
        (&mut f).take(limit as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() != len as usize {
            return Err("metadata torn/changed".into());
        }
        if self.instrument {
            self.metrics.bytes_read += bytes.len() as u64;
        }
        Ok(bytes)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
struct Root {
    format: u32,
    variant: String,
    seq: u64,
    node: Option<Id>,
    parent: Option<Id>,
    owner: String,
    lock: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RootEnvelope {
    root: Root,
    checksum: Id,
}

fn load_root(o: &mut Objects, variant: &str, owner: &str) -> Result<(Root, Id)> {
    let bytes = o.read_record(&o.base.join("ROOT"), 4096)?;
    let env: RootEnvelope = serde_json::from_slice(&bytes)?;
    let raw = serde_json::to_vec(&env.root)?;
    if hash(&raw) != env.checksum
        || env.root.format != 1
        || env.root.variant != variant
        || env.root.owner != owner
    {
        return Err("root checksum/owner/variant mismatch".into());
    }
    if env.root.node.as_ref().is_some_and(|x| !valid_id(x))
        || env.root.parent.as_ref().is_some_and(|x| !valid_id(x))
    {
        return Err("invalid root references".into());
    }
    // Positively link selected metadata to a retained immutable root object.
    let root_object = o.read(&hash_frame(&raw))?;
    if root_object != raw {
        return Err("root link mismatch".into());
    }
    Ok((env.root, env.checksum))
}
fn hash_frame(bytes: &[u8]) -> Id {
    let mut framed = MAGIC.to_vec();
    framed.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    framed.extend_from_slice(bytes);
    hash(&framed)
}

fn publish(o: &mut Objects, root: &Root, expected: Option<&Id>) -> Result<Id> {
    let raw = serde_json::to_vec(root)?;
    let id = o.write(&raw)?;
    o.flush_names()?;
    let sum = hash(&raw);
    let bytes = serde_json::to_vec(&RootEnvelope {
        root: root.clone(),
        checksum: sum.clone(),
    })?;
    let stage = o
        .base
        .join(format!("ROOT-stage-{}-{}", std::process::id(), root.seq));
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&stage)?;
    f.write_all(&bytes)?;
    checkpoint("root_stage")?;
    f.sync_all()?;
    checkpoint("root_sync")?;
    drop(f);
    if let Some(exp) = expected {
        if load_root(o, &root.variant, &root.owner)?.1 != *exp {
            return Err("expected root conflict under lock".into());
        }
    } else if o.base.join("ROOT").exists() {
        return Err("initial root already exists".into());
    }
    replace(&stage, &o.base.join("ROOT"), expected.is_some())?;
    checkpoint("root_replace")?;
    sync_directory(&o.base)?;
    // Also flush the selected root handle. Windows doesn't imply parent fsync.
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(o.base.join("ROOT"))?
        .sync_all()?;
    checkpoint("publication_sync")?;
    if o.instrument {
        o.metrics.bytes_written += bytes.len() as u64;
        o.metrics.syncs += 2;
        o.metrics.replacements += 1;
    }
    let _ = id;
    Ok(sum)
}

#[derive(Clone)]
enum Index {
    Chunk(chunk::ChunkIndex),
    Disk(disk::DiskIndex),
}
impl Index {
    fn open(o: &mut Objects, v: &str, r: Option<Id>) -> Result<Self> {
        Ok(match v {
            "chunk" => Self::Chunk(chunk::ChunkIndex::open(o, r)?),
            "disk" => Self::Disk(disk::DiskIndex::open(o, r)?),
            _ => return Err("unknown variant".into()),
        })
    }
    fn bulk(o: &mut Objects, v: &str, p: Vec<(String, Vec<u8>)>) -> Result<Self> {
        Ok(match v {
            "chunk" => Self::Chunk(chunk::ChunkIndex::bulk(o, p)?),
            "disk" => Self::Disk(disk::DiskIndex::bulk(o, p)?),
            _ => return Err("unknown variant".into()),
        })
    }
    fn get(&mut self, o: &mut Objects, k: &str) -> Result<Option<Vec<u8>>> {
        match self {
            Self::Chunk(x) => x.get(o, k),
            Self::Disk(x) => x.get(o, k),
        }
    }
    fn put(&mut self, o: &mut Objects, k: String, v: Vec<u8>) -> Result<()> {
        match self {
            Self::Chunk(x) => x.put(o, k, v),
            Self::Disk(x) => x.put(o, k, v),
        }
    }
    fn persist(&mut self, o: &mut Objects) -> Result<Option<Id>> {
        match self {
            Self::Chunk(x) => x.persist(o),
            Self::Disk(x) => x.persist(o),
        }
    }
    fn audit(&mut self, o: &mut Objects) -> Result<usize> {
        match self {
            Self::Chunk(x) => x.audit(o),
            Self::Disk(x) => x.audit(o),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
struct Request {
    request: String,
    item: String,
    content: String,
    proof: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Retained {
    original: Request,
    state: String,
    outcome: Option<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Authority {
    epoch: u64,
    scope: bool,
}
fn get_json<T: serde::de::DeserializeOwned>(
    idx: &mut Index,
    o: &mut Objects,
    k: &str,
) -> Result<Option<T>> {
    idx.get(o, k)?
        .map(|b| serde_json::from_slice(&b).map_err(Into::into))
        .transpose()
}

fn apply_request(idx: &mut Index, o: &mut Objects, c: &Value) -> Result<(Value, bool)> {
    let req = Request {
        request: field(c, "request")?,
        item: field(c, "item")?,
        content: field(c, "content")?,
        proof: c["proof"].as_u64().ok_or("proof required")?,
    };
    for s in [&req.request, &req.item, &req.content] {
        if s.is_empty() || s.len() > 64 {
            return Err("request field limit 1..64 UTF-8 bytes".into());
        }
    }
    let stage = c["stage"].as_str().unwrap_or("terminal");
    if !["reserve", "accept", "attempt", "terminal"].contains(&stage) {
        return Err("unknown stage".into());
    }
    let key = format!("r:{}", req.request);
    let old: Option<Retained> = get_json(idx, o, &key)?;
    if let Some(old) = &old {
        if old.original != req {
            return Ok((
                json!({"status":"refused","reason":"request differs from exact original"}),
                false,
            ));
        }
        if old.state == "terminal" {
            return Ok((json!({"status":"replay","outcome":old.outcome}), false));
        }
    }
    let auth: Authority = get_json(idx, o, "a:current")?.ok_or("authority missing")?;
    if !auth.scope || auth.epoch != req.proof {
        return Ok((
            json!({"status":"refused","reason":"current original authority absent"}),
            false,
        ));
    }
    if let Some(old) = &old {
        if old.state == "attempted" {
            return Ok((
                json!({"status":"uncertain","reason":"possible attempt without terminal; reconciliation required"}),
                false,
            ));
        }
        if old.state == "accepted" && stage == "reserve" {
            return Ok((
                json!({"status":"refused","reason":"state cannot regress"}),
                false,
            ));
        }
    }
    let item_key = format!("i:{}", req.item);
    let item: Option<String> = get_json(idx, o, &item_key)?;
    if item.as_ref().is_some_and(|x| x != &req.request) {
        return Ok((
            json!({"status":"refused","reason":"item already reserved"}),
            false,
        ));
    }
    if old.is_some() && item.is_none() {
        return Err("request/item membership inconsistent".into());
    }
    let state = match stage {
        "reserve" => "reserved",
        "accept" => "accepted",
        "attempt" => "attempted",
        _ => "terminal",
    };
    let outcome = (state == "terminal").then(|| format!("outcome:{}:{}", req.request, req.content));
    let rec = Retained {
        original: req.clone(),
        state: state.into(),
        outcome: outcome.clone(),
    };
    idx.put(o, key, serde_json::to_vec(&rec)?)?;
    idx.put(o, item_key, serde_json::to_vec(&req.request)?)?;
    Ok((
        json!({"status":match state {"terminal"=>"committed","attempted"=>"uncertain",x=>x},"outcome":outcome}),
        true,
    ))
}
fn field(c: &Value, k: &str) -> Result<String> {
    Ok(c[k].as_str().ok_or_else(|| format!("{k} required"))?.into())
}

fn ready(c: &Value) -> Result<()> {
    if let Some(p) = c["ready_file"].as_str() {
        fs::write(p, b"prepared")?;
    }
    std::thread::sleep(Duration::from_millis(
        c["pause_ms"].as_u64().unwrap_or(0).min(60000),
    ));
    Ok(())
}

fn execute(o: &mut Objects, v: &str, c: &Value) -> Result<Value> {
    o.instrument = c["instrument"].as_bool().unwrap_or(true);
    o.guard_list = c["guard"].as_str() == Some("list");
    let dir = directory(&o.base)?;
    let owner = identity(&dir)?;
    let op = c["op"].as_str().ok_or("op required")?;
    let lock_path = o.base.join("LOCK");
    if op != "init" && !lock_path.exists() {
        return Err("established lock missing; no repair".into());
    }
    if lock_path.exists() {
        plain(&lock_path)?;
    }
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)?;
    let lock_id = identity(&lock)?;
    if op == "init" {
        let wait = Instant::now();
        lock.lock_exclusive()?;
        o.metrics.lock_wait_ns = wait.elapsed().as_nanos() as u64;
        let held = Instant::now();
        if o.base.join("OWNER").exists() || o.base.join("ROOT").exists() {
            return Err("store already established; never rebuild from absence".into());
        }
        let mut marker = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(o.base.join("OWNER"))?;
        marker.write_all(owner.as_bytes())?;
        marker.sync_all()?;
        sync_directory(&o.base)?;
        let n = c["n"].as_u64().ok_or("n required")?;
        if n > 1_000_000 {
            return Err("experimental size limit".into());
        }
        let mut pairs = vec![(
            "a:current".into(),
            serde_json::to_vec(&Authority {
                epoch: 1,
                scope: true,
            })?,
        )];
        for i in 0..n {
            pairs.push((format!("u:{i:09}"), vec![b'x'; 96]));
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let mut idx = Index::bulk(o, v, pairs)?;
        let root = Root {
            format: 1,
            variant: v.into(),
            seq: 0,
            node: idx.persist(o)?,
            parent: None,
            owner,
            lock: lock_id,
        };
        publish(o, &root, None)?;
        o.metrics.lock_hold_ns = held.elapsed().as_nanos() as u64;
        return Ok(json!({"status":"initialized","seq":0,"n":n}));
    }
    let marker = o.read_record(&o.base.join("OWNER"), 256)?;
    if marker != owner.as_bytes() {
        return Err("native root identity changed".into());
    }
    let opening = Instant::now();
    let (mut root, mut expected) = load_root(o, v, &owner)?;
    if root.lock != lock_id {
        return Err("native established lock identity changed".into());
    }
    let mut idx = Index::open(o, v, root.node.clone())?;
    let open_ns = opening.elapsed().as_nanos() as u64;
    if ["lookup", "hold", "snapshot", "audit"].contains(&op) {
        if op == "snapshot" {
            return Ok(serde_json::to_value(&root)?);
        }
        if op == "audit" {
            let pairs = idx.audit(o)?;
            let mut objects = 0u64;
            let object_dir = o.base.join("objects");
            o.guard(&object_dir)?;
            o.metrics.lists += 1;
            for shard in fs::read_dir(object_dir)? {
                let shard = shard?;
                o.metrics.listed_entries += 1;
                if !plain(&shard.path())?.is_dir() {
                    return Err("invalid object namespace entry".into());
                }
                o.metrics.lists += 1;
                for entry in fs::read_dir(shard.path())? {
                    let entry = entry?;
                    o.metrics.listed_entries += 1;
                    let id = entry.file_name().to_string_lossy().to_string();
                    if valid_id(&id) {
                        o.read(&id)?;
                        objects += 1;
                    } else if !id.starts_with("stage-") {
                        return Err("unexpected object namespace entry".into());
                    }
                }
            }
            return Ok(json!({"pairs":pairs,"objects":objects,"seq":root.seq}));
        }
        ready(c)?;
        let keys = c["keys"].as_array().ok_or("keys required")?;
        if keys.len() > 4096 {
            return Err("lookup key count limit".into());
        }
        let repeats = c["repeat"].as_u64().unwrap_or(1);
        if repeats == 0 || repeats > 10000 {
            return Err("repeat limit".into());
        }
        let mut values = Vec::new();
        let core = Instant::now();
        for _ in 0..repeats {
            values.clear();
            for k in keys {
                let k = k.as_str().ok_or("string key required")?;
                if k.len() > 1024 {
                    return Err("lookup key length".into());
                }
                values.push(idx.get(o, k)?.map(|b| {
                    serde_json::from_slice::<Value>(&b)
                        .unwrap_or_else(|_| json!(String::from_utf8_lossy(&b)))
                }));
            }
        }
        return Ok(
            json!({"values":values,"seq":root.seq,"core_ns":core.elapsed().as_nanos() as u64,"open_ns":open_ns,"repeat":repeats}),
        );
    }
    if !["run", "authority", "put"].contains(&op) {
        return Err("unknown command".into());
    }
    // Preparation snapshot retained without the writer lock, including eager A cost.
    ready(c)?;
    let wait = Instant::now();
    lock.lock_exclusive()?;
    o.metrics.lock_wait_ns = wait.elapsed().as_nanos() as u64;
    let held = Instant::now();
    // Native root and lock must still refer to the retained objects under coordination.
    o.guard(&o.base.clone())?;
    if identity(&directory(&o.base)?)? != owner || identity(&File::open(&lock_path)?)? != lock_id {
        return Err("native root/lock identity changed".into());
    }
    let (current, current_id) = load_root(o, v, &owner)?;
    if current.lock != lock_id {
        return Err("native selected lock identity changed".into());
    }
    if current_id != expected {
        o.metrics.retries += 1;
        root = current;
        expected = current_id;
        idx = Index::open(o, v, root.node.clone())?;
    }
    let (mut result, changed) = match op {
        "run" => apply_request(&mut idx, o, c)?,
        "authority" => {
            let auth: Authority = get_json(&mut idx, o, "a:current")?.ok_or("authority missing")?;
            let epoch = c["epoch"].as_u64().ok_or("epoch required")?;
            if epoch <= auth.epoch {
                return Err("authority epoch must increase".into());
            }
            idx.put(
                o,
                "a:current".into(),
                serde_json::to_vec(&Authority {
                    epoch,
                    scope: c["scope"].as_bool().ok_or("scope required")?,
                })?,
            )?;
            (json!({"status":"authority_changed"}), true)
        }
        "put" => {
            let pairs = c["pairs"].as_array().ok_or("pairs required")?;
            if pairs.is_empty() || pairs.len() > 256 {
                return Err("batch pair limit".into());
            }
            for pair in pairs {
                let k = pair[0].as_str().ok_or("key required")?;
                let value = pair[1].as_str().ok_or("string value required")?;
                if !k.starts_with("u:") {
                    return Err("benchmark put may only touch unrelated keys".into());
                }
                idx.put(o, k.into(), value.as_bytes().to_vec())?;
            }
            (json!({"status":"committed","pairs":pairs.len()}), true)
        }
        _ => unreachable!(),
    };
    if changed {
        let next = Root {
            format: 1,
            variant: v.into(),
            seq: root.seq.checked_add(1).ok_or("sequence overflow")?,
            node: idx.persist(o)?,
            parent: Some(hash_frame(&serde_json::to_vec(&root)?)),
            owner,
            lock: lock_id,
        };
        publish(o, &next, Some(&expected))?;
        root = next;
    }
    result["seq"] = json!(root.seq);
    o.metrics.lock_hold_ns = held.elapsed().as_nanos() as u64;
    fs2::FileExt::unlock(&lock)?;
    checkpoint("before_reply")?;
    Ok(result)
}

fn peak_rss() -> Option<u64> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::{
            ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
            Threading::GetCurrentProcess,
        };
        let mut info: PROCESS_MEMORY_COUNTERS = unsafe { std::mem::zeroed() };
        let n = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
        info.cb = n;
        if unsafe { GetProcessMemoryInfo(GetCurrentProcess(), &mut info, n) } != 0 {
            return Some(info.PeakWorkingSetSize as u64 / 1024);
        }
    }
    #[cfg(target_os = "linux")]
    {
        let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) } == 0 {
            return Some(ru.ru_maxrss as u64);
        }
    }
    #[cfg(target_os = "macos")]
    {
        let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
        if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut ru) } == 0 {
            return Some(ru.ru_maxrss as u64 / 1024);
        }
    }
    None
}

fn main() {
    let started = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    let read = std::io::stdin()
        .take(1024 * 1024 + 1)
        .read_to_string(&mut input);
    if args.len() != 3 || read.is_err() || input.len() > 1024 * 1024 {
        println!(
            "{}",
            json!({"ok":false,"error":"usage: membership-prototype STORE chunk|disk; JSON stdin <=1MiB"})
        );
        return;
    }
    let mut o = match Objects::new(PathBuf::from(&args[1])) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", json!({"ok":false,"error":e.to_string()}));
            return;
        }
    };
    let res = serde_json::from_str::<Value>(&input)
        .map_err(Into::into)
        .and_then(|c| execute(&mut o, &args[2], &c));
    let mut out = match res {
        Ok(r) => json!({"ok":true,"result":r}),
        Err(e) => json!({"ok":false,"error":e.to_string()}),
    };
    out["metrics"] = serde_json::to_value(&o.metrics).unwrap();
    out["elapsed_ns"] = json!(started.elapsed().as_nanos() as u64);
    out["peak_rss_kib"] = json!(peak_rss());
    println!("{out}");
}
