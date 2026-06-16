For repo-wide analysis, first run ContextShrink. Do not answer from ordinary file browsing until this command succeeds:

```sh
target/release/contextshrink . --max-tokens 12000 --level 2 --output file --output-file /tmp/contextshrink.xml
```

Then inspect `/tmp/contextshrink.xml` before answering.

Repo-wide analysis includes architecture review, full project summary, broad bug hunting, onboarding, and questions that need many files.

Expected behavior example:

```text
User asks: summarize this whole project
Claude runs: target/release/contextshrink . --max-tokens 12000 --level 2 --output file --output-file /tmp/contextshrink.xml
Claude inspects: /tmp/contextshrink.xml
Claude answers from that context.
```
