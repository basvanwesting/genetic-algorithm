---
name: contrib-review
description: Review the queue of external pull requests and issues on this repo (tachsin's batches and anyone else's) one PR at a time with low machine load. Use whenever the user asks to review, triage, check, or merge incoming PRs or issues, mentions "the queue", "new PRs", "tachsin", or asks what to do with contributions. Do not review PRs ad hoc with parallel agents; use this workflow.
---

# Contribution review, marathon mode

External contributors file PRs faster than they can be merged. Reviewing them in
parallel once cost 47 GB of cargo target dirs and 13 concurrent rustc jobs. This
workflow is serial on purpose: one worktree, one shared target dir, one PR at a
time. It runs in the background while the maintainer does other things and is
revisited several times a day.

State lives in `~/.local/state/ga-review/` (override with `GA_REVIEW_DIR`):
`ledger.tsv` (verdict per PR, survives sessions), `reports/pr-N.md`, the reused
worktree `wt/` and cargo `target/`. Contributor plumbing to rely on: tachsin
bases every PR on current main, links each to an issue with "Fixes #N", and
rebases his own open PRs after merges. So merge conflicts are his problem, not
ours: skip a conflicting PR and pick it up next pass.

## 1. Queue

```
.claude/skills/contrib-review/scripts/queue.sh
```

Lists open PRs with size, mergeable state, linked issue and ledger verdict
(`NEW` = never looked at), then issues with no PR. Nothing here builds.

## 2. Triage, no builds

For each `NEW` PR read only the issue and PR body (`gh issue view`, `gh pr view`)
and bucket:

- **fix**: a crash, hang, wrong result or contract violation, with a regression
  test. Goes to step 3.
- **design**: changes results of seeded runs, adds API or config surface, adds
  platforms or CI, changes semantics (e.g. "avoid no-op swaps", "point 0 in
  crossover", "support i64"). The maintainer decides whether it is wanted before
  anyone spends a build on it. Present these as a list with the one-line
  tradeoff and stop.
- **docs/ci**: read the diff, verify claims against the code, no build.
- **issue only**: needs an answer from the maintainer, not a review. Summarize
  the question and a recommended answer.

Report the buckets to the user first. Only the fix and docs buckets proceed
without a decision.

## 3. Mechanical review, one PR at a time

```
.claude/skills/contrib-review/scripts/review-pr.sh <N>
```

Never run two at once. The script fetches the PR, proves the bug by running the
PR's new test functions against main's source (FAIL or HANG on main means
confirmed), then runs the full suite, clippy, fmt and examples on the PR branch,
checks for conflicts with main, and flags conventions: `///` on non-pub items,
new pub items, edited test expectations, files outside src/tests. Report goes to
stdout and `reports/pr-N.md`; the worktree stays at `wt/` for reading.

Then judge, either inline or with ONE subagent (sequentially, never a fan-out).
The judgment is what the script cannot do:

- Is the fix minimal and in the crate's idiom? Compare with how siblings do it
  (the other genotypes, the other strategies, the other reporters). Builders
  report errors via the existing `TryFromBuilderError`; new error types or
  `try_new` constructors for programmer misconfiguration are not wanted.
- Does it change results for configs that worked before? Seeded expectations
  may only change with a justification in the PR body. The rng stream must stay
  identical for unaffected paths.
- Supporting machinery: new pub fns, new fields, widened signatures. Private
  helpers used in one file stay private. Count them as cost.
- Anything outside the stated scope: Cargo.lock regeneration, CI, CHANGELOG
  (the maintainer writes CHANGELOG at release time), dependency changes.
- Does the issue text match the code paths it cites?

Subagent prompt, when used: give it the report path, the diff (`git diff
origin/main pr-N`), the worktree path, the judgment list above, and a 150-word
cap. It must not run cargo again; the script already did.

## 4. Verdict and ledger

```
.claude/skills/contrib-review/scripts/ledger.sh <N> MERGE|CHANGES|REJECT|DISCUSS "note"
```

`CHANGES` means small requests before merge (name them in the note). `DISCUSS`
means the design bucket awaiting the maintainer. When the maintainer merges or
closes, append `MERGED` or `CLOSED` so the queue shows it.

## 5. Report to the user

One table: PR, issue, bucket, verdict, one-line reason. Then the design
questions needing a decision, then follow-ups reviewers found that no PR covers.
Merge steps at the end: `gh pr merge N --rebase` in number order for every MERGE;
CHANGES PRs after the contributor updates them; skip anything the queue shows as
CONFLICTING, the contributor rebases it. After a merge batch the maintainer runs
the suite on main and writes the CHANGELOG entries.

## Learned conventions (batch 1 and 2, Sept 2026)

- History is linear; always `--rebase` merge. Commits keep the contributor's
  authorship.
- `//` for private items, `///` only for public API docs.
- Real bugs so far: panics in builders and hot paths at edge configs, HashMap
  iteration breaking seeded determinism, missing trait impls that siblings have
  (flush), stale docs after deliberate behaviour changes. All were confirmed by
  the test failing against main's source.
- Rejected so far: PR #10, a dependency swap that broke seeded determinism and
  lost exact counts, with a regenerated lockfile. Motivation was the
  contributor's own crate, not a measured bottleneck here.
- Contributor asks good questions in issue bodies (e.g. "should operator
  constructors return errors?"). Answer them in the verdict note; default is to
  keep panics for programmer misconfiguration.

## Cleanup

`review-pr.sh --clean` removes the worktree, `pr-*` branches and the target dir
(about 2 GB). Ledger and reports stay. Run it when a batch is done.
