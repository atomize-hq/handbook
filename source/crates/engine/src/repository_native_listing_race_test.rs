//! Deterministic Linux listing-window test over a disposable source bundle.
use super::*;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::{Duration, Instant};

thread_local! {
    static SOURCE_LISTING_HOOK_V1: RefCell<Option<Box<dyn FnOnce()>>> = RefCell::new(None);
}
struct ListingHookGuardV1 {
    previous: Option<Box<dyn FnOnce()>>,
    _not_send: PhantomData<Rc<()>>,
}
impl Drop for ListingHookGuardV1 {
    fn drop(&mut self) { SOURCE_LISTING_HOOK_V1.with(|slot| { slot.replace(self.previous.take()); }); }
}
fn install_source_listing_hook_v1(hook: impl FnOnce() + 'static) -> ListingHookGuardV1 {
    let previous = SOURCE_LISTING_HOOK_V1.with(|slot| slot.replace(Some(Box::new(hook))));
    ListingHookGuardV1 { previous, _not_send: PhantomData }
}
pub(super) fn after_source_listing_v1() {
    let hook = SOURCE_LISTING_HOOK_V1.with(|slot| slot.borrow_mut().take());
    if let Some(hook) = hook { hook(); }
}
struct WatchedListingChildV1(std::process::Child);
impl Drop for WatchedListingChildV1 {
    fn drop(&mut self) { let _ = self.0.kill(); let _ = self.0.wait(); }
}

#[test]
fn listing_post_observation_mutation_is_refused() {
    const ROOT: &str = "HANDBOOK_TASK19_LISTING_RACE_ROOT";
    if let Some(root) = std::env::var_os(ROOT) {
        let root = PathBuf::from(root);
        let bytes = b"source-unit-member";
        let admission = RepositoryNativeManifestAdmissionV1 { members: vec![RepositoryNativeManifestMemberV1 {
            path: format!(".handbook/state/dock/local-evaluation-v1/packages/{}/bin/validator", "0".repeat(64)),
            length: bytes.len() as u64,
            sha256: Sha256::digest(bytes).into(),
        }] };
        let source = open_production_source_root_v1(&root).unwrap();
        let fired = Rc::new(Cell::new(false));
        let fired_in_hook = Rc::clone(&fired);
        let extra = root.join("after-list-extra");
        let _hook = install_source_listing_hook_v1(move || {
            fs::write(extra, b"appeared-after-listing").unwrap();
            fired_in_hook.set(true);
        });
        assert!(source.retain_production_source_inventory_v1(&admission).is_err());
        assert!(fired.get(), "the post-list/pre-witness window was exercised");
        return;
    }
    let fixture = tempfile::Builder::new().prefix("task19-listing-race-").tempdir().unwrap();
    let bundle = fixture.path().join("bundle");
    fs::create_dir_all(bundle.join("bin")).unwrap();
    fs::write(bundle.join("bin/validator"), b"source-unit-member").unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("repository_durability::native_listing_race_test::listing_post_observation_mutation_is_refused")
        .arg("--nocapture").env(ROOT, &bundle).spawn().unwrap();
    let mut watched = WatchedListingChildV1(child);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = watched.0.try_wait().unwrap() {
            assert!(status.success(), "listing child failed: {status}");
            break;
        }
        assert!(Instant::now() < deadline, "listing test child exceeded its watchdog");
        std::thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(fs::read(bundle.join("after-list-extra")).unwrap(), b"appeared-after-listing");
}

#[test]
fn listing_hook_restores_on_panic_and_nesting() {
    let outer_fired = Rc::new(Cell::new(false));
    let observed = Rc::clone(&outer_fired);
    let outer = install_source_listing_hook_v1(move || observed.set(true));
    let panic = std::panic::catch_unwind(|| {
        let _inner = install_source_listing_hook_v1(|| panic!("test listing hook panic"));
        after_source_listing_v1();
    });
    assert!(panic.is_err());
    after_source_listing_v1();
    assert!(outer_fired.get());
    drop(outer);
    after_source_listing_v1();
}
