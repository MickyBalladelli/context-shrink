# TODO

## Tests

- Add parser snapshot tests for:
  - Rust skeleton output
  - TypeScript skeleton output
  - Python skeleton output
  - tree map output
- Add budget tests for downgrade behavior:
  - full to skeleton
  - skeleton to tree map
  - stable file ordering
- Add formatter tests for XML escaping.
- Add walker tests for ignored files and supported extensions.

## Install Story

- Support install with:

```sh
cargo install --path .
```

- Update plugins and VS Code extension to prefer the installed `contextshrink` binary.
- Add one clear smoke test for each integration:
  - CLI
  - Codex plugin
  - Claude Code plugin
  - VS Code extension

## Plugins

- Make Codex and Claude instructions more explicit about when ContextShrink must run.
- Add expected behavior examples:
  - ask for whole project summary
  - see ContextShrink command execute
  - inspect `/tmp/contextshrink.xml`
- Add marketplace/install docs for Claude Code if publishing.

## CLI Quality

- Add `--include` and `--exclude` flags.
- Add `--respect-gitignore` toggle.
- Add `--print-files` to show selected files.
- Add `--fail-on-empty` for automation.
- Add clearer errors when no supported files are found.

## Output Quality

- Add metadata to XML:
  - generated time
  - repo root
  - token budget
  - compression level
  - file count
- Add per-file token counts.
- Add a short project map section before file contents.
- Consider JSON output as an alternative to XML.
