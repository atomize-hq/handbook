//! Focused Linux source-bundle FIFO tests. Fixtures are synthetic unit data,
//! never replacements for the production validator or its qualification.
use super::*;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::{Duration, Instant};

thread_local! {
    static SOURCE_REOPEN_HOOK_V1: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None);
}

struct SourceReopenHookGuard {
    previous: Option<Box<dyn FnOnce()>>,
    _not_send: PhantomData<Rc<()>>,
}

impl Drop for SourceReopenHookGuard {
    fn drop(&mut self) {
        SOURCE_REOPEN_HOOK_V1.with(|slot| { slot.replace(self.previous.take()); });
    }
}

fn install_source_reopen_hook_v1(hook: impl FnOnce() + 'static) -> SourceReopenHookGuard {
    let previous = SOURCE_REOPEN_HOOK_V1.with(|slot| slot.replace(Some(Box::new(hook))));
    SourceReopenHookGuard { previous, _not_send: PhantomData }
}

pub(super) fn before_source_named_reopen_v1() {
    let hook = SOURCE_REOPEN_HOOK_V1.with(|slot| slot.borrow_mut().take());
    if let Some(hook) = hook { hook(); }
}

struct WatchedSourceChild(std::process::Child);
impl Drop for WatchedSourceChild {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn synthetic_source_admission_v1(bytes: &[u8]) -> RepositoryNativeManifestAdmissionV1 {
    RepositoryNativeManifestAdmissionV1 { members: vec![RepositoryNativeManifestMemberV1 {
        path: format!(".handbook/state/dock/local-evaluation-v1/packages/{}/bin/validator", "0".repeat(64)),
        length: bytes.len() as u64,
        sha256: Sha256::digest(bytes).into(),
    }] }
}

fn make_source_fifo_v1(path: &Path) {
    rustix::fs::mkfifoat(rustix::fs::CWD, path, rustix::fs::Mode::from_bits_truncate(0o600)).unwrap();
}

fn run_source_fifo_child_v1(case: &str, bundle: &Path) {
    let path = bundle.join("bin/validator");
    let bytes = b"bounded-source-unit-fixture";
    let admission = synthetic_source_admission_v1(bytes);
    if case == "initial" {
        make_source_fifo_v1(&path);
        let root = open_production_source_root_v1(bundle).unwrap();
        assert!(matches!(root.read_production_source_member_v1(&admission, "bin/validator"),
            Err(RepositoryNativeSourceFailureV1::Unsafe)));
    } else {
        assert_eq!(case, "reopen");
        fs::write(&path, bytes).unwrap();
        let root = open_production_source_root_v1(bundle).unwrap();
        let (_, file) = root.read_production_source_member_v1(&admission, "bin/validator").unwrap();
        let fired = Rc::new(Cell::new(false));
        let fired_in_hook = Rc::clone(&fired);
        let _hook = install_source_reopen_hook_v1(move || {
            fs::remove_file(&path).unwrap();
            make_source_fifo_v1(&path);
            fired_in_hook.set(true);
        });
        assert!(matches!(file.validate_production_source_file_v1(),
            Err(RepositoryNativeSourceFailureV1::Unsafe)));
        assert!(fired.get(), "the named-reopen race was actually exercised");
    }
}

#[test]
fn source_fifo_initial_and_reopen_are_nonblocking() {
    const CASE: &str = "HANDBOOK_TASK19_SOURCE_FIFO_CASE";
    const ROOT: &str = "HANDBOOK_TASK19_SOURCE_FIFO_ROOT";
    if let Ok(case) = std::env::var(CASE) {
        run_source_fifo_child_v1(&case, Path::new(&std::env::var_os(ROOT).unwrap()));
        return;
    }
    for case in ["initial", "reopen"] {
        // Parent owns the fixture and cleans it after the child has exited or
        // has been killed and reaped by the watchdog.
        let fixture = tempfile::Builder::new().prefix("task19-source-fifo-").tempdir().unwrap();
        let bundle = fixture.path().join("bundle");
        fs::create_dir_all(bundle.join("bin")).unwrap();
        let child = std::process::Command::new(std::env::current_exe().unwrap())
            .arg("--exact")
            .arg("repository_durability::native_source_tests::source_fifo_initial_and_reopen_are_nonblocking")
            .arg("--nocapture")
            .env(CASE, case).env(ROOT, &bundle).spawn().unwrap();
        let mut watched = WatchedSourceChild(child);
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if let Some(status) = watched.0.try_wait().unwrap() {
                assert!(status.success(), "{case} FIFO child failed: {status}");
                break;
            }
            assert!(Instant::now() < deadline, "{case} FIFO open blocked");
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

#[test]
fn reopen_hook_restores_after_nesting_and_panic() {
    let outer_fired = Rc::new(Cell::new(false));
    let observed = Rc::clone(&outer_fired);
    let outer = install_source_reopen_hook_v1(move || observed.set(true));
    let panic = std::panic::catch_unwind(|| {
        let _inner = install_source_reopen_hook_v1(|| panic!("test hook panic"));
        before_source_named_reopen_v1();
    });
    assert!(panic.is_err());
    before_source_named_reopen_v1();
    assert!(outer_fired.get());
    drop(outer);
    before_source_named_reopen_v1();
}
