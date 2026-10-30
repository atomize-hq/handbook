//! Focused native tests through the reusable public API. No research command
//! protocol or malformed/corrupt-store continuation is used here.
use file_native::*;
use std::sync::{Arc, Barrier};
fn create(dir: &std::path::Path) -> Database<NativeBackend> {
    Database::create(NativeBackend::open(dir).unwrap(), "test-owner").unwrap()
}
fn put(db: &Database<NativeBackend>, id: &str, key: &str, value: &[u8]) -> OriginalOutcome {
    let mut tx = Transaction::new(db.version().unwrap(), id, id.as_bytes().to_vec());
    tx.put(key, value.to_vec()).unwrap();
    db.publish(tx).unwrap()
}
#[test]
fn native_publication_originals_snapshots_and_reopen() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    let mut tx = Transaction::new(
        db.version().unwrap(),
        "first",
        b"exact original outcome".to_vec(),
    );
    tx.put("alpha", b"original value".to_vec()).unwrap();
    let first = db.publish(tx.clone()).unwrap();
    let old = db.snapshot().unwrap();
    put(&db, "second", "alpha", b"successor");
    assert_eq!(old.get("alpha").unwrap(), Some(b"original value".to_vec()));
    assert_eq!(db.publish(tx).unwrap(), first);
    assert_eq!(
        db.resolve("first").unwrap(),
        OriginalResolution::Resolved(first.clone())
    );
    assert_eq!(db.get("alpha").unwrap(), Some(b"successor".to_vec()));
    assert!(
        db.maintain(MaintenanceBudget::default())
            .unwrap()
            .blocked_by_reader
    );
    drop(old);
    let mut removed = 0;
    loop {
        let p = db.maintain(MaintenanceBudget::default()).unwrap();
        removed += p.removed_files;
        if p.complete {
            break;
        }
    }
    assert!(removed > 0);
    drop(db);
    let db = Database::open(NativeBackend::open(dir.path()).unwrap(), "test-owner").unwrap();
    assert_eq!(
        db.resolve("first").unwrap(),
        OriginalResolution::Resolved(first)
    );
    assert_eq!(db.get("alpha").unwrap(), Some(b"successor".to_vec()));
    assert_eq!(
        db.statistics().unwrap().directory_sync_supported,
        cfg!(unix)
    );
}
#[test]
fn bounded_pages_cross_leaf_branches_tombstones_and_container_history() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    for batch in 0usize..70 {
        let mut tx = Transaction::new(
            db.version().unwrap(),
            format!("batch-{batch}"),
            vec![batch as u8],
        );
        for n in 0..64 {
            let k = batch * 64 + n;
            tx.put(format!("key-{k:05}"), k.to_le_bytes().to_vec())
                .unwrap();
        }
        db.publish(tx).unwrap();
    }
    // More than 64 selected containers and the old 4096 discovery interval.
    let mut tx = Transaction::new(db.version().unwrap(), "deletes", vec![]);
    for n in 0..256 {
        tx.delete(format!("key-{n:05}")).unwrap();
    }
    db.publish(tx).unwrap();
    let snap = db.snapshot().unwrap();
    let mut next = None;
    let mut seen = Vec::new();
    let mut pages = 0;
    loop {
        let p = snap.page(next.as_deref(), 31).unwrap();
        pages += 1;
        seen.extend(p.records);
        if p.complete {
            break;
        }
        next = p.next;
        assert!(next.is_some());
    }
    assert!(pages > 128);
    assert_eq!(seen.len(), 70 * 64 - 256);
    for (n, (k, v)) in seen.iter().enumerate() {
        let n = n + 256;
        assert_eq!(k, &format!("key-{n:05}"));
        assert_eq!(v, &n.to_le_bytes());
    }
    assert_eq!(db.statistics().unwrap().version.records, seen.len() as u64);
}
#[test]
fn four_callers_four_readers_and_one_maintainer_preserve_old_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let db = Arc::new(create(dir.path()));
    put(&db, "seed", "shared", b"seed");
    let old = db.snapshot().unwrap();
    let barrier = Arc::new(Barrier::new(9));
    let mut jobs = Vec::new();
    let version = db.version().unwrap();
    for n in 0..4 {
        let db = db.clone();
        let barrier = barrier.clone();
        let version = version.clone();
        jobs.push(std::thread::spawn(move || {
            barrier.wait();
            let mut tx = Transaction::new(version, format!("caller-{n}"), vec![n]);
            tx.put(format!("concurrent-{n}"), vec![n]).unwrap();
            db.publish(tx).unwrap()
        }));
    }
    let mut readers = Vec::new();
    for _ in 0..3 {
        let db = db.clone();
        let barrier = barrier.clone();
        readers.push(std::thread::spawn(move || {
            let s = db.snapshot().unwrap();
            barrier.wait();
            for _ in 0..8 {
                assert_eq!(s.get("shared").unwrap(), Some(b"seed".to_vec()));
            }
        }));
    }
    let mdb = db.clone();
    let mbar = barrier.clone();
    let maintainer = std::thread::spawn(move || {
        mbar.wait();
        for _ in 0..8 {
            mdb.maintain(MaintenanceBudget::default()).unwrap();
        }
    });
    barrier.wait();
    for _ in 0..8 {
        assert_eq!(old.get("shared").unwrap(), Some(b"seed".to_vec()));
    }
    let outcomes: Vec<_> = jobs.into_iter().map(|j| j.join().unwrap()).collect();
    for r in readers {
        r.join().unwrap();
    }
    maintainer.join().unwrap();
    assert_eq!(
        outcomes
            .iter()
            .filter(|o| o.disposition == OriginalDisposition::Selected)
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|o| o.disposition == OriginalDisposition::Conflict)
            .count(),
        3
    );
    drop(old);
    loop {
        if db.maintain(MaintenanceBudget::default()).unwrap().complete {
            break;
        }
    }
    put(&db, "after-drain", "fresh", b"continued");
}
#[test]
fn reader_admission_is_active_and_releases_without_a_lifetime_counter() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    let pins: Vec<_> = (0..MAX_READERS).map(|_| db.snapshot().unwrap()).collect();
    assert!(db.snapshot().is_err());
    drop(pins);
    for _ in 0..MAX_READERS * 3 {
        drop(db.snapshot().unwrap());
    }
    put(&db, "continued", "value", b"yes");
}
#[cfg(feature = "test-support")]
#[test]
fn interrupted_owned_outputs_and_selection_recover_without_selecting_an_unselected_effect() {
    for boundary in [
        PublicationBoundary::Owned,
        PublicationBoundary::Outputs,
        PublicationBoundary::BeforeSelection,
        PublicationBoundary::SelectorDisplaced,
        PublicationBoundary::Selected,
        PublicationBoundary::Reply,
    ] {
        let dir = tempfile::tempdir().unwrap();
        let db = create(dir.path());
        let mut tx = Transaction::new(db.version().unwrap(), "original", b"outcome".to_vec());
        tx.put("item", b"effect".to_vec()).unwrap();
        assert!(db
            .publish_with_boundary(tx, |at| if at == boundary {
                Err("interrupted".into())
            } else {
                Ok(())
            })
            .is_err());
        drop(db);
        let db =
            Database::open_for_recovery(NativeBackend::open(dir.path()).unwrap(), "test-owner")
                .unwrap();
        db.recover().unwrap();
        let OriginalResolution::Resolved(out) = db.resolve("original").unwrap() else {
            panic!("positive original evidence required")
        };
        let selected = matches!(
            boundary,
            PublicationBoundary::Selected | PublicationBoundary::Reply
        );
        assert_eq!(
            out.disposition,
            if selected {
                OriginalDisposition::Selected
            } else {
                OriginalDisposition::NotSelected
            }
        );
        assert_eq!(
            db.get("item").unwrap(),
            if selected {
                Some(b"effect".to_vec())
            } else {
                None
            }
        );
        put(&db, "successor", "fresh", b"continued after recovery");
    }
}

#[test]
fn native_noreplace_preserves_an_existing_target_and_open_reader() {
    let dir = tempfile::tempdir().unwrap();
    let io = NativeBackend::open(dir.path()).unwrap();
    io.write_new("source", b"source").unwrap();
    io.write_new("target", b"target").unwrap();
    let held = std::fs::File::open(dir.path().join("target")).unwrap();
    assert!(io.rename_new("source", "target").is_err());
    assert_eq!(io.read("source", 10).unwrap(), Some(b"source".to_vec()));
    assert_eq!(io.read("target", 10).unwrap(), Some(b"target".to_vec()));
    io.rename_new("source", "fresh").unwrap();
    assert_eq!(io.read("fresh", 10).unwrap(), Some(b"source".to_vec()));
    drop(held);
}

#[cfg(feature = "test-support")]
#[test]
fn native_creation_resumes_only_its_original_positive_empty_initialization() {
    for boundary in [
        CreationBoundary::Binding,
        CreationBoundary::Store,
        CreationBoundary::Locks,
        CreationBoundary::Selector,
        CreationBoundary::Ready,
    ] {
        let dir = tempfile::tempdir().unwrap();
        let io = NativeBackend::open(dir.path()).unwrap();
        assert!(
            Database::create_with_boundary(io, "creation-owner", |at| if at == boundary {
                Err("creation interrupted".into())
            } else {
                Ok(())
            })
            .is_err()
        );
        let db =
            Database::resume_creation(NativeBackend::open(dir.path()).unwrap(), "creation-owner")
                .unwrap();
        put(&db, "after-create", "live", b"preserved");
        assert!(Database::resume_creation(
            NativeBackend::open(dir.path()).unwrap(),
            "creation-owner"
        )
        .is_err());
        let reopened =
            Database::open(NativeBackend::open(dir.path()).unwrap(), "creation-owner").unwrap();
        assert_eq!(reopened.get("live").unwrap(), Some(b"preserved".to_vec()));
    }
}
#[test]
fn retired_bytes_continue_across_fixed_batches_and_old_reader_release() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    for n in 0..132 {
        put(
            &db,
            &format!("seed-{n}"),
            &format!("large-{n:04}"),
            &vec![1; 512 * 1024],
        );
    }
    let pin = db.snapshot().unwrap();
    for n in 0..132 {
        put(
            &db,
            &format!("replace-{n}"),
            &format!("large-{n:04}"),
            &vec![2; 512 * 1024],
        );
    }
    let mut before = 0;
    loop {
        let p = db
            .maintain(MaintenanceBudget {
                items: 7,
                bytes: 1024 * 1024,
            })
            .unwrap();
        before += p.removed_bytes;
        if p.blocked_by_reader {
            break;
        }
    }
    assert_eq!(pin.get("large-0000").unwrap(), Some(vec![1; 512 * 1024]));
    drop(pin);
    let mut removed = 0;
    let mut batches = 0;
    loop {
        let p = db
            .maintain(MaintenanceBudget {
                items: 7,
                bytes: 1024 * 1024,
            })
            .unwrap();
        removed += p.removed_bytes;
        batches += 1;
        assert!(p.visited <= 7);
        if p.complete {
            break;
        }
    }
    assert!(removed > 64 * 1024 * 1024);
    assert!(batches > 64);
    assert_eq!(db.get("large-0000").unwrap(), Some(vec![2; 512 * 1024]));
    assert!(matches!(
        db.resolve("seed-0").unwrap(),
        OriginalResolution::Resolved(OriginalOutcome {
            disposition: OriginalDisposition::Selected,
            ..
        })
    ));
    println!("retired_source_bytes={removed} batches={batches} earlier_cleanup_bytes={before}");
}
#[test]
fn publication_and_retirement_continue_beyond_4096_original_outcomes() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    for n in 0u64..4101 {
        put(&db, &format!("history-{n:05}"), "current", &n.to_le_bytes());
        if n % 128 == 0 {
            db.maintain(MaintenanceBudget {
                items: 4096,
                bytes: 64 * 1024 * 1024,
            })
            .unwrap();
        }
    }
    loop {
        if db.maintain(MaintenanceBudget::default()).unwrap().complete {
            break;
        }
    }
    let db = Database::open(NativeBackend::open(dir.path()).unwrap(), "test-owner").unwrap();
    assert_eq!(db.statistics().unwrap().version.indexed_originals, 4101);
    for n in [0, 4095, 4100] {
        assert!(matches!(
            db.resolve(&format!("history-{n:05}")).unwrap(),
            OriginalResolution::Resolved(OriginalOutcome {
                disposition: OriginalDisposition::Selected,
                ..
            })
        ));
    }
    put(&db, "beyond-history", "fresh", b"continued");
}
#[test]
fn native_reader_process_death_releases_retirement_without_expiring_a_live_lease() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    put(&db, "process-seed", "process-key", b"old");
    let ready = dir.path().join("child-ready");
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "native_reader_child", "--nocapture"])
        .env("FILE_NATIVE_READER_CHILD", dir.path())
        .env("FILE_NATIVE_READY", &ready)
        .spawn()
        .unwrap();
    let start = std::time::Instant::now();
    while !ready.exists() {
        assert!(start.elapsed() < std::time::Duration::from_secs(10));
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    put(&db, "process-successor", "process-key", b"new");
    assert!(
        db.maintain(MaintenanceBudget::default())
            .unwrap()
            .blocked_by_reader
    );
    child.kill().unwrap();
    child.wait().unwrap();
    loop {
        if db.maintain(MaintenanceBudget::default()).unwrap().complete {
            break;
        }
    }
    assert_eq!(db.get("process-key").unwrap(), Some(b"new".to_vec()));
}
#[test]
fn native_reader_child() {
    let Some(path) = std::env::var_os("FILE_NATIVE_READER_CHILD") else {
        return;
    };
    let db = Database::open(NativeBackend::open(path).unwrap(), "test-owner").unwrap();
    let snapshot = db.snapshot().unwrap();
    assert_eq!(snapshot.get("process-key").unwrap(), Some(b"old".to_vec()));
    std::fs::write(std::env::var_os("FILE_NATIVE_READY").unwrap(), b"pinned").unwrap();
    loop {
        std::thread::park();
    }
}

#[test]
fn equal_values_at_distinct_keys_keep_independent_retirement_ownership() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    let mut tx = Transaction::new(db.version().unwrap(), "same-values", vec![]);
    tx.put("left", b"same value".to_vec()).unwrap();
    tx.put("right", b"same value".to_vec()).unwrap();
    db.publish(tx).unwrap();
    put(&db, "replace-left", "left", b"replacement");
    while !db.maintain(MaintenanceBudget::default()).unwrap().complete {}
    assert_eq!(db.get("right").unwrap(), Some(b"same value".to_vec()));
    assert_eq!(db.get("left").unwrap(), Some(b"replacement".to_vec()));
}

#[test]
fn exact_maximum_payload_survives_framed_read_pin_replacement_and_retirement() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    let maximum = vec![85; MAX_OBJECT];
    put(&db, "maximum-original", "maximum", &maximum);
    let old = db.snapshot().unwrap();
    assert_eq!(
        old.get("maximum").unwrap().as_ref().map(Vec::len),
        Some(MAX_OBJECT)
    );
    put(&db, "maximum-successor", "maximum", &vec![86; MAX_OBJECT]);
    assert_eq!(old.get("maximum").unwrap(), Some(maximum));
    drop(old);
    let mut removed = 0;
    loop {
        let p = db.maintain(MaintenanceBudget::default()).unwrap();
        removed += p.removed_bytes;
        if p.complete {
            break;
        }
    }
    assert!(removed > MAX_OBJECT as u64);
    assert_eq!(db.get("maximum").unwrap(), Some(vec![86; MAX_OBJECT]));
}
#[cfg(feature = "test-support")]
#[test]
fn native_publisher_death_reconciles_owned_outputs_and_displaced_selection() {
    for stage in ["outputs", "displaced"] {
        let dir = tempfile::tempdir().unwrap();
        let db = create(dir.path());
        let ready = dir.path().join("publisher-ready");
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "native_publisher_child", "--nocapture"])
            .env("FILE_NATIVE_PUBLISHER_CHILD", dir.path())
            .env("FILE_NATIVE_READY", &ready)
            .env("FILE_NATIVE_PAUSE", stage)
            .spawn()
            .unwrap();
        let start = std::time::Instant::now();
        while !ready.exists() {
            assert!(start.elapsed() < std::time::Duration::from_secs(10));
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        child.kill().unwrap();
        child.wait().unwrap();
        drop(db);
        let db =
            Database::open_for_recovery(NativeBackend::open(dir.path()).unwrap(), "test-owner")
                .unwrap();
        db.recover().unwrap();
        assert_eq!(db.get("process-effect").unwrap(), None);
        assert!(matches!(
            db.resolve("process-original").unwrap(),
            OriginalResolution::Resolved(OriginalOutcome {
                disposition: OriginalDisposition::NotSelected,
                ..
            })
        ));
        put(
            &db,
            "after-process-death",
            "process-effect",
            b"fresh effect",
        );
        assert_eq!(
            db.get("process-effect").unwrap(),
            Some(b"fresh effect".to_vec())
        );
    }
}
#[cfg(feature = "test-support")]
#[test]
fn native_publisher_child() {
    let Some(path) = std::env::var_os("FILE_NATIVE_PUBLISHER_CHILD") else {
        return;
    };
    let db = Database::open(NativeBackend::open(path).unwrap(), "test-owner").unwrap();
    let mut tx = Transaction::new(
        db.version().unwrap(),
        "process-original",
        b"original outcome".to_vec(),
    );
    tx.put("process-effect", b"same bytes".to_vec()).unwrap();
    tx.put("process-other", b"same bytes".to_vec()).unwrap();
    let pause = std::env::var("FILE_NATIVE_PAUSE").unwrap();
    db.publish_with_boundary(tx, |at| {
        if (pause == "outputs" && at == PublicationBoundary::Outputs)
            || (pause == "displaced" && at == PublicationBoundary::SelectorDisplaced)
        {
            std::fs::write(std::env::var_os("FILE_NATIVE_READY").unwrap(), b"owned").unwrap();
            loop {
                std::thread::park();
            }
        }
        Ok(())
    })
    .unwrap();
}

#[derive(Clone)]
struct CountingBackend {
    inner: NativeBackend,
    writes: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
impl Storage for CountingBackend {
    fn validate(&self) -> Result<()> {
        self.inner.validate()
    }
    fn read(&self, p: &str, n: usize) -> Result<Option<Vec<u8>>> {
        self.inner.read(p, n)
    }
    fn mkdir(&self, p: &str) -> Result<()> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.mkdir(p)
    }
    fn write_new(&self, p: &str, b: &[u8]) -> Result<()> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.write_new(p, b)
    }
    fn rename_new(&self, a: &str, b: &str) -> Result<()> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.rename_new(a, b)
    }
    fn displace(&self, a: &str, b: &[u8], c: &str) -> Result<()> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.displace(a, b, c)
    }
    fn remove(&self, p: &str) -> Result<()> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.inner.remove(p)
    }
    fn directory_sync_supported(&self) -> bool {
        self.inner.directory_sync_supported()
    }
}
impl Backend for CountingBackend {
    type Guard = <NativeBackend as Backend>::Guard;
    fn lock(&self, p: &str) -> Result<Self::Guard> {
        self.inner.lock(p)
    }
    fn try_lock(&self, p: &str) -> Result<Option<Self::Guard>> {
        self.inner.try_lock(p)
    }
    fn validate_lock(&self, p: &str, g: &Self::Guard) -> Result<()> {
        self.inner.validate_lock(p, g)
    }
    fn lock_identity(&self, g: &Self::Guard) -> Result<String> {
        self.inner.lock_identity(g)
    }
}
#[test]
fn current_view_writes_no_pin_and_excludes_publish_and_maintenance_for_full_scope() {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    };
    let dir = tempfile::tempdir().unwrap();
    let writes = Arc::new(AtomicUsize::new(0));
    let io = CountingBackend {
        inner: NativeBackend::open(dir.path()).unwrap(),
        writes: writes.clone(),
    };
    let db = Database::create(io, "current-view-owner").unwrap();
    let mut seed = Transaction::new(db.version().unwrap(), "seed", vec![]);
    seed.put("key", b"old".to_vec()).unwrap();
    db.publish(seed).unwrap();
    let pin_before = std::fs::read(dir.path().join("readers/0.json")).unwrap();
    writes.store(0, Ordering::Relaxed);
    for _ in 0..16 {
        let (version, value) = db
            .with_current(|view| (view.version().clone(), view.get("key").unwrap()))
            .unwrap();
        assert_eq!(version.records, 1);
        assert_eq!(value, Some(b"old".to_vec()));
    }
    assert_eq!(writes.load(Ordering::Relaxed), 0);
    assert_eq!(
        std::fs::read(dir.path().join("readers/0.json")).unwrap(),
        pin_before
    );
    let expected = db.version().unwrap();
    let (started, entered) = mpsc::channel();
    let (finished, completed) = mpsc::channel();
    let mut workers = Vec::new();
    db.with_current(|view| {
        let writer = db.clone();
        let txstart = started.clone();
        let txdone = finished.clone();
        workers.push(std::thread::spawn(move || {
            let mut tx = Transaction::new(expected, "writer", vec![]);
            tx.put("key", b"new".to_vec()).unwrap();
            txstart.send(()).unwrap();
            writer.publish(tx).unwrap();
            txdone.send("writer").unwrap();
        }));
        let maintainer = db.clone();
        let txstart = started.clone();
        let txdone = finished.clone();
        workers.push(std::thread::spawn(move || {
            txstart.send(()).unwrap();
            maintainer.maintain(MaintenanceBudget::default()).unwrap();
            txdone.send("maintainer").unwrap();
        }));
        entered.recv().unwrap();
        entered.recv().unwrap();
        assert!(matches!(
            completed.recv_timeout(std::time::Duration::from_millis(20)),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
        assert_eq!(view.get("key").unwrap(), Some(b"old".to_vec()));
    })
    .unwrap();
    for worker in workers {
        worker.join().unwrap();
    }
    let mut names = vec![completed.recv().unwrap(), completed.recv().unwrap()];
    names.sort();
    assert_eq!(names, vec!["maintainer", "writer"]);
    assert_eq!(db.get("key").unwrap(), Some(b"new".to_vec()));
}
#[test]
fn current_view_callback_error_and_unwind_release_the_native_gate() {
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    let result: std::result::Result<(), &str> = db.with_current(|_| Err("caller refusal")).unwrap();
    assert_eq!(result, Err("caller refusal"));
    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        db.with_current(|_| panic!("intentional consuming callback unwind"))
            .unwrap();
    }));
    assert!(unwind.is_err());
    put(&db, "after-unwind", "key", b"gate released");
    assert_eq!(db.get("key").unwrap(), Some(b"gate released".to_vec()));
}

#[test]
fn borrowed_selection_guard_refusal_preserves_original_outcome_and_native_order() {
    #[derive(Debug)]
    struct Refusal(u8);
    impl std::fmt::Display for Refusal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "original borrowed guard refusal {}", self.0)
        }
    }
    impl std::error::Error for Refusal {}
    for refuse_at in 1..=5 {
        let dir = tempfile::tempdir().unwrap();
        let db = create(dir.path());
        let predecessor = db.version().unwrap();
        let mut tx = Transaction::new(predecessor.clone(), "guarded", b"exact outcome".to_vec());
        tx.put("effect", b"exact payload".to_vec()).unwrap();
        let mut calls = 0;
        let result = db.publish_guarded(tx.clone(), || {
            calls += 1;
            let cell:serde_json::Value=serde_json::from_slice(&std::fs::read(dir.path().join("active.json"))?)?;
            if calls==1 {
                assert_eq!(cell["state"],"idle");
                assert_eq!(cell["value"]["version"]["sequence"],predecessor.sequence);
            }else{
                assert_eq!(cell["state"],"owned");
                let owned=cell["value"]["positive"]["owned"].as_array().unwrap();
                assert!(!owned.is_empty());
                for id in owned {let id=id.as_str().unwrap();assert!(dir.path().join(format!("objects/{}/{}/{}",&id[..2],&id[2..4],id)).is_file());}
            }
            // Old after-output/displaced/post-selection calls1/2/3 are now2/3/4.
            assert_eq!(dir.path().join("SELECTED.json").is_file(),calls!=3);
            assert_eq!(dir.path().join("SELECTED.json.displaced").is_file(),calls==3);
            if calls == refuse_at {
                return Err(Box::new(Refusal(calls)));
            }
            Ok(())
        });
        if refuse_at <= 4 {
            let error = result.unwrap_err().downcast::<Refusal>().unwrap();
            assert_eq!(error.0, refuse_at);
            assert_eq!(calls, refuse_at);
        } else {
            assert_eq!(result.unwrap().disposition, OriginalDisposition::Selected);
            assert_eq!(calls, 4);
        }
        drop(db);
        let db =
            Database::open_for_recovery(NativeBackend::open(dir.path()).unwrap(), "test-owner")
                .unwrap();
        db.recover().unwrap();
        if refuse_at==1 {
            assert_eq!(db.resolve("guarded").unwrap(),OriginalResolution::Absent);
            assert_eq!(db.get("effect").unwrap(),None);
            assert_eq!(db.version().unwrap(),predecessor);
            assert_eq!(db.publish(tx).unwrap().disposition,OriginalDisposition::Selected);
            continue;
        }
        let OriginalResolution::Resolved(original) = db.resolve("guarded").unwrap() else {
            panic!("exact original after guarded publication");
        };
        let selected = refuse_at >= 4;
        assert_eq!(
            original.disposition,
            if selected {
                OriginalDisposition::Selected
            } else {
                OriginalDisposition::NotSelected
            }
        );
        assert_eq!(
            original.value,
            if selected {
                b"exact outcome".to_vec()
            } else {
                Vec::new()
            }
        );
        assert_eq!(
            db.get("effect").unwrap(),
            if selected {
                Some(b"exact payload".to_vec())
            } else {
                None
            }
        );
        assert_eq!(
            db.publish_guarded(tx, || panic!("replay cannot consume current permission"))
                .unwrap(),
            original
        );
        let idle:serde_json::Value=serde_json::from_slice(&std::fs::read(dir.path().join("active.json")).unwrap()).unwrap();
        assert_eq!(idle["state"],"idle");
        put(&db, "fresh-after-guard", "successor", b"continued");
        assert_eq!(db.get("successor").unwrap(), Some(b"continued".to_vec()));
    }
}

#[cfg(feature = "test-support")]
#[test]
fn t03_d02_real_create_prefix_interruption_recovers_initialized_store() {
    struct OwnedChild(std::process::Child);
    impl Drop for OwnedChild {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let db = create(dir.path());
    put(&db, "seed", "seed", b"unchanged");
    let ready = dir.path().join("created-ready");
    let mut child = OwnedChild(
        std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "t03_d02_create_prefix_child", "--nocapture"])
            .env("T03_CREATE_CHILD", dir.path())
            .env("T03_CREATE_READY", &ready)
            .spawn()
            .unwrap(),
    );
    let started = std::time::Instant::now();
    while !ready.exists() {
        assert!(
            child.0.try_wait().unwrap().is_none(),
            "child exited before actual create"
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(10),
            "actual create boundary not reached"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let relative = std::fs::read_to_string(&ready).unwrap();
    assert_eq!(
        std::fs::metadata(dir.path().join(&relative)).unwrap().len(),
        0
    );
    println!("actual exclusive create before first write: {relative}");
    child.0.kill().unwrap();
    let status = child.0.wait().unwrap();
    assert!(!status.success());
    drop(db);
    let reopened =
        Database::open_for_recovery(NativeBackend::open(dir.path()).unwrap(), "test-owner")
            .unwrap();
    reopened
        .recover()
        .expect("valid initialized-store create-prefix interruption must recover");
    assert_eq!(reopened.get("seed").unwrap(), Some(b"unchanged".to_vec()));
    put(&reopened, "after-create-death", "continued", b"yes");
    while !reopened
        .maintain(MaintenanceBudget::default())
        .unwrap()
        .complete
    {}
    drop(reopened);
    let reopened = Database::open(NativeBackend::open(dir.path()).unwrap(), "test-owner").unwrap();
    assert_eq!(reopened.get("continued").unwrap(), Some(b"yes".to_vec()));
}
#[cfg(feature = "test-support")]
#[test]
fn t03_d02_create_prefix_child() {
    let Some(path) = std::env::var_os("T03_CREATE_CHILD") else {
        return;
    };
    let ready = std::env::var_os("T03_CREATE_READY").unwrap();
    let backend = NativeBackend::open(&path)
        .unwrap()
        .with_create_observer(move |relative| {
            if relative == "active.json" || relative == "active.json.prepared.writing" {
                std::fs::write(&ready, relative)?;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
            Ok(())
        });
    let db = Database::open(backend, "test-owner").unwrap();
    let mut tx = Transaction::new(db.version().unwrap(), "interrupted", b"outcome".to_vec());
    tx.put("interrupted-effect", b"effect".to_vec()).unwrap();
    db.publish(tx).unwrap();
}

#[cfg(feature="test-support")]
#[test]
fn retained_opaque_evidence_precedes_outputs_and_survives_unrelated_publication() {
    let dir=tempfile::tempdir().unwrap();let db=create(dir.path());
    put(&db,"seed-evidence","user",b"predecessor");
    let predecessor=db.version().unwrap();
    let payload=b"EXACT borrowed domain checkpoint bytes\nterminal.json";
    let mut tx=Transaction::new(predecessor.clone(),"attempt-owned",b"exact-outcome".to_vec());
    tx.put("user",b"candidate effect".to_vec()).unwrap();tx.set_evidence("stable-scope",payload.to_vec()).unwrap();
    let mut owned_proof=None;
    let result=db.publish_with_boundary(tx.clone(),|at|{
        if at==PublicationBoundary::Owned {
            let cell:serde_json::Value=serde_json::from_slice(&std::fs::read(dir.path().join("active.json"))?)?;
            assert_eq!(cell["state"],"owned");
            let positive=&cell["value"]["positive"];
            let proof:RetainedEvidence=serde_json::from_value(positive["original"]["evidence"].clone())?;
            assert_eq!(proof.payload,payload);assert_eq!(proof.scope,"stable-scope");
            assert_eq!(proof.binding.expected,predecessor);assert_eq!(proof.binding.predecessor,predecessor);
            assert_eq!(proof.binding.key,"attempt-owned");
            assert_eq!(proof.binding.candidate.sequence,predecessor.sequence+1);
            assert_eq!(proof.binding.candidate.token,positive["candidate"]["token"].as_str().unwrap());
            for id in positive["owned"].as_array().unwrap(){let id=id.as_str().unwrap();assert!(!dir.path().join(format!("objects/{}/{}/{}",&id[..2],&id[2..4],id)).exists());}
            let negative:RetainedEvidence=serde_json::from_value(cell["value"]["negative"]["original"]["evidence"].clone())?;
            assert_eq!(negative.payload,proof.payload);assert_eq!(negative.binding.candidate,proof.binding.candidate);
            assert_ne!(negative.binding.resolving,proof.binding.resolving);
            owned_proof=Some(proof);
            return Err("valid owned-before-outputs interruption".into());
        }Ok(())
    });
    assert!(result.is_err());db.recover().unwrap();
    let negative=db.reconcile_evidence("stable-scope",None).unwrap();
    let original=negative.original.unwrap();
    assert_eq!(original.outcome,OriginalOutcome{disposition:OriginalDisposition::NotSelected,sequence:predecessor.sequence,value:Vec::new()});
    let mut prior=owned_proof.unwrap();prior.binding.resolving=original.evidence.binding.resolving.clone();
    assert_eq!(original.evidence,prior);
    assert_eq!(db.get("user").unwrap(),Some(b"predecessor".to_vec()));
    assert_eq!(db.publish(tx).unwrap(),original.outcome);
    // Unrelated publication invalidates seq-1 discovery, while exact scope/key
    // lookup still reaches this retained original without a history walk.
    put(&db,"unrelated","other",b"value");
    let mut next=Transaction::new(db.version().unwrap(),"scope-successor",b"new-outcome".to_vec());
    next.put("user",b"new valid effect".to_vec()).unwrap();next.set_evidence("stable-scope",b"later checkpoint".to_vec()).unwrap();
    let selected=db.publish(next).unwrap();assert_eq!(selected.disposition,OriginalDisposition::Selected);
    let newest=db.reconcile_evidence("stable-scope",None).unwrap().original.unwrap();
    assert_eq!(newest.evidence.previous.as_ref(),Some(&original.reference));
    while !db.maintain(MaintenanceBudget::default()).unwrap().complete{}
    drop(db);let db=Database::open(NativeBackend::open(dir.path()).unwrap(),"test-owner").unwrap();
    let older=db.reconcile_evidence("stable-scope",Some("attempt-owned")).unwrap().original.unwrap();
    assert_eq!(older.reference,original.reference);assert_eq!(older.evidence,original.evidence);assert_eq!(older.outcome,original.outcome);
    assert_eq!(db.reconcile_evidence("stable-scope",None).unwrap().original.unwrap().reference,newest.reference);
}
#[cfg(feature="test-support")]
#[test]
fn initialized_controls_real_create_deaths_preserve_old_pin_and_continue() {
    struct Child(std::process::Child);impl Drop for Child{fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
    for role in ["owned","idle","selector","event","cursor","reader","negative"] {
        let dir=tempfile::tempdir().unwrap();let db=create(dir.path());put(&db,"control-seed","seed",b"old pinned bytes");
        let old=db.snapshot().unwrap();let ready=dir.path().join("control-ready");
        let mut child=Child(std::process::Command::new(std::env::current_exe().unwrap()).args(["--exact","initialized_control_create_child","--nocapture"]).env("T03_CONTROL_CHILD",dir.path()).env("T03_CONTROL_ROLE",role).env("T03_CONTROL_READY",&ready).spawn().unwrap());
        let start=std::time::Instant::now();while !ready.exists(){assert!(child.0.try_wait().unwrap().is_none(),"child exited for {role}");assert!(start.elapsed()<std::time::Duration::from_secs(10),"unreached create {role}");std::thread::sleep(std::time::Duration::from_millis(5));}
        let relative=std::fs::read_to_string(&ready).unwrap();assert_eq!(std::fs::metadata(dir.path().join(&relative)).unwrap().len(),0);
        println!("actual initialized {role} exclusive-create-before-write {relative}");
        child.0.kill().unwrap();assert!(!child.0.wait().unwrap().success());
        let reopened=Database::open_for_recovery(NativeBackend::open(dir.path()).unwrap(),"test-owner").unwrap();reopened.recover().unwrap();
        assert_eq!(old.get("seed").unwrap(),Some(b"old pinned bytes".to_vec()));
        let original=reopened.resolve("control-attempt").unwrap();
        if role=="owned"{assert_eq!(original,OriginalResolution::Absent);}
        else {let OriginalResolution::Resolved(outcome)=original else{panic!("original lost {role}")};assert_eq!(outcome.disposition,if ["selector","negative"].contains(&role){OriginalDisposition::NotSelected}else{OriginalDisposition::Selected});}
        put(&reopened,"after-control-death","seed",b"continued");
        assert!(reopened.maintain(MaintenanceBudget::default()).unwrap().blocked_by_reader);
        assert_eq!(old.get("seed").unwrap(),Some(b"old pinned bytes".to_vec()));drop(old);
        if role=="reader" {
            // Actual child death left a partial reserved pin staging file. All
            // fixed native slots must be reusable without adopting that file.
            let pins:Vec<_>=(0..MAX_READERS).map(|_|reopened.snapshot().unwrap()).collect();
            for pin in &pins{assert_eq!(pin.get("seed").unwrap(),Some(b"continued".to_vec()));}
            assert!(reopened.snapshot().is_err());drop(pins);
        }
        while !reopened.maintain(MaintenanceBudget::default()).unwrap().complete{}
        drop(reopened);drop(db);
        let final_db=Database::open(NativeBackend::open(dir.path()).unwrap(),"test-owner").unwrap();assert_eq!(final_db.get("seed").unwrap(),Some(b"continued".to_vec()));
    }
}
#[cfg(feature="test-support")]
#[test]
fn initialized_control_create_child(){
    let Some(path)=std::env::var_os("T03_CONTROL_CHILD")else{return};
    let role=std::env::var("T03_CONTROL_ROLE").unwrap();let ready=std::env::var_os("T03_CONTROL_READY").unwrap();
    let seen=std::sync::atomic::AtomicUsize::new(0);let recovering=Arc::new(std::sync::atomic::AtomicBool::new(false));let recover_flag=recovering.clone();let mode=role.clone();
    let backend=NativeBackend::open(&path).unwrap().with_create_observer(move|relative|{
        let matching=match mode.as_str(){
            "owned"|"idle"=>relative=="active.json.prepared.writing",
            "selector"=>relative=="SELECTED.json.prepared.writing",
            "event"=>relative.starts_with("retirements/")&&relative.ends_with(".writing"),
            "cursor"=>relative=="maintenance.json.prepared.writing",
            "reader"=>relative.starts_with("readers/")&&relative.ends_with(".json.prepared.writing"),
            "negative"=>recover_flag.load(std::sync::atomic::Ordering::SeqCst)&&relative.starts_with("objects/")&&relative.ends_with(".writing"),
            _=>false,
        };
        if matching {
            let n=seen.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
            if mode!="idle"||n==1 {
                std::fs::write(&ready,relative)?;
                loop{std::thread::sleep(std::time::Duration::from_millis(50));}
            }
        }Ok(())
    });
    let db=Database::open(backend,"test-owner").unwrap();
    let mut tx=Transaction::new(db.version().unwrap(),"control-attempt",b"outcome".to_vec());tx.put("seed",b"new selected bytes".to_vec()).unwrap();
    if role=="negative" {
        assert!(db.publish_with_boundary(tx,|at|if at==PublicationBoundary::Owned{Err("valid interruption".into())}else{Ok(())}).is_err());
        recovering.store(true,std::sync::atomic::Ordering::SeqCst);db.recover().unwrap();
    }else{
        db.publish(tx).unwrap();
        if role=="cursor"{db.maintain(MaintenanceBudget::default()).unwrap();}
        if role=="reader"{let _pin=db.snapshot().unwrap();panic!("observer not reached");}
    }
}
