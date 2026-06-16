# ContextShrink

VS Code extension that generates ContextShrink XML for GitHub Copilot Chat and Codex.

Install into VS Code:

```sh
"/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code" --install-extension copilot/contextshrink-vscode/contextshrink-vscode-0.1.0.vsix
```

Install into Cursor:

```sh
code --install-extension copilot/contextshrink-vscode/contextshrink-vscode-0.1.0.vsix
```

On some machines, `code` points to Cursor. Use the full VS Code path when you want Visual Studio Code.

Install the CLI first:

```sh
cargo install --path .
```

The extension checks `CONTEXTSHRINK_BIN`, then `contextshrink` on `PATH`, then local release builds.

Use Command Palette:

```text
ContextShrink: Generate Context
```

or:

```text
ContextShrink: Generate and Ask
```

or:

```text
ContextShrink: Copy Context Prompt
```

or:

```text
ContextShrink: Preview Project Map
```

or:

```text
ContextShrink: Copy Project Map
```
