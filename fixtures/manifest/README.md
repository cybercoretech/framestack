# Manifest fixtures

Shared `framestack.toml` examples that every FrameStack implementation must agree on ([ADR-0008](../../docs/decisions/0008-manifest-and-tooling.md)). The Rust core runs them in `crates/framestack-core/tests/fixtures.rs`.

| Folder | Contents | Expected result |
| --- | --- | --- |
| `valid/` | Manifests with no problems, including the three reference stacks from ADR-0007 | No diagnostics |
| `invalid/` | One problem per file | Only the stated error code |
| `warning/` | Valid manifests with one reported issue | Valid, with only the stated warning code |

The first line of every fixture states what it expects:

```toml
# expect: valid
# expect: error FS007
# expect: warning FS015
```

To add a fixture, put it in the right folder with that first line, and keep it to one problem so a failure points at one rule. The codes are listed in the [manifest reference](../../docs/reference/manifest.md#validation-rules).
