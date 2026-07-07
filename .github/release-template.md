# ${RELEASE_TAG}

## Quickstart

- **Install the latest tagged release without building locally:**

  ```bash
  curl -fsSL https://raw.githubusercontent.com/atomize-hq/handbook/${RELEASE_TAG}/scripts/handbook/install.sh | bash
  ```

- **Install a specific version:**

  ```bash
  curl -fsSL https://raw.githubusercontent.com/atomize-hq/handbook/${RELEASE_TAG}/scripts/handbook/install.sh | bash -s -- --version ${VERSION}
  ```

- **Repo-local install from source checkout:**

  ```bash
  cargo install --locked --force --path crates/cli
  bash tools/codex/install.sh
  ```

## Bundles

| OS | Arch | Bundle |
| --- | --- | --- |
| Linux | x86_64 | [handbook-v${VERSION}-linux_x86_64.tar.gz](https://github.com/atomize-hq/handbook/releases/download/${RELEASE_TAG}/handbook-v${VERSION}-linux_x86_64.tar.gz) |
| macOS | arm64 | [handbook-v${VERSION}-macos_arm64.tar.gz](https://github.com/atomize-hq/handbook/releases/download/${RELEASE_TAG}/handbook-v${VERSION}-macos_arm64.tar.gz) |

Each bundle contains the exact curated `~/handbook/` home expected by the shipped Codex packaging contract, including `~/handbook/bin/handbook`, `runtime-manifest.json`, `resources/**`, the installed thin projections under `~/handbook/.agents/skills/*`, and refreshed Codex discovery links under `~/.codex/skills/*`.

## Checksums

- [SHA256SUMS](https://github.com/atomize-hq/handbook/releases/download/${RELEASE_TAG}/SHA256SUMS)
- Verify on Linux/macOS: `curl -fsSLO https://github.com/atomize-hq/handbook/releases/download/${RELEASE_TAG}/SHA256SUMS && shasum -a 256 -c SHA256SUMS`

---

This release intentionally publishes only the currently supported public targets: `macOS arm64` and `Linux x86_64`.
