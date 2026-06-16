For repo-wide analysis, first run ContextShrink. This is required for full project summaries, architecture review, onboarding, broad bug hunting, and any question that needs many files. Do not answer from ordinary file browsing until this command succeeds:

```sh
target/release/contextshrink . --max-tokens 12000 --level 2 --output file --output-file /tmp/contextshrink.xml
```

Then inspect `/tmp/contextshrink.xml` before answering.

Expected behavior example:

```text
User asks: summarize this whole project
You run: target/release/contextshrink . --max-tokens 12000 --level 2 --output file --output-file /tmp/contextshrink.xml
You inspect: /tmp/contextshrink.xml
Then answer from that context.
```
