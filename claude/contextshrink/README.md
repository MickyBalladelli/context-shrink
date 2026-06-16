# ContextShrink Claude Code Plugin

Claude Code plugin for generating token-budgeted ContextShrink XML before broad repo analysis.

Install the CLI first:

```sh
cargo install --path .
```

The helper checks `CONTEXTSHRINK_BIN`, then `contextshrink` on `PATH`, then this repo's release binary.

Test from the repo root:

```sh
claude --plugin-dir ./claude/contextshrink
```

Use in Claude Code:

```text
/contextshrink:contextshrink
```

The skill writes `/tmp/contextshrink.xml`, then Claude reads it before answering.

Expected behavior:

```text
Ask: summarize this whole project
See: ContextShrink command execute
Inspect: /tmp/contextshrink.xml
Answer: summary uses compressed repository context
```

Marketplace install from this repo root:

```sh
claude plugin marketplace add .
```

Then install in Claude Code:

```text
/plugin install contextshrink@context-shrink
```

Validate before publishing:

```sh
claude plugin validate .
```

Examples:

```sh
claude/contextshrink/bin/contextshrink-claude src 12000 2 /tmp/contextshrink-src.xml
claude/contextshrink/bin/contextshrink-claude . 12000 2 /tmp/contextshrink.xml --exclude '**/generated/**'
claude/contextshrink/bin/contextshrink-claude . 12000 2 /tmp/contextshrink.json --format json
```

Release checklist:

```text
cargo test
cargo build --release
claude plugin validate .
claude/contextshrink/bin/contextshrink-claude . 12000 2 /tmp/contextshrink.xml
bump claude/contextshrink/.claude-plugin/plugin.json version for pinned releases
```
