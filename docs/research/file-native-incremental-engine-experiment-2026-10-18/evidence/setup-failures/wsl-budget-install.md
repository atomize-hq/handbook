# WSL budget-adapter setup failure

The first host installer invocation exited1 with `AssertionError` while expecting
21 Rust paths. Its Git query used the repository-relative prefix from the
experiment working directory and returned no paths. The original installer is
preserved beside this note. It had copied the four original helper files before
installing wrappers; no engine, benchmark or native case had been launched.

The corrected query runs from the Git root with `--full-tree`. Verification then
checked all21 Rust files and all four preserved original scripts against7ac,
compiled each wrapper, and checked the exact budget-adapter hash. The install
receipt records those hashes. A separate three-second process probe confirmed
Driver deadline clamping, termination of the adapter's own descendant group and
explicit unperformed output when the original work window is expired.

The original source TAR and Rust bytes remain unchanged. This is a pre-execution
harness setup failure, not an engine observation or a reset native window.
