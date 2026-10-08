---
name: contrib-review
description: Interactive, one-item-at-a-time handling of external pull requests and issues on this repo (tachsin's batches and anyone else's). The maintainer says "next item" (or "next", "review the queue", names a PR or issue number, or says "go"); Claude analyzes, advises, prepares the action, and executes only after the maintainer's go. Use for anything about incoming PRs, issues, the queue, tachsin, merging or answering contributions. Never fan out parallel reviewer agents.
---

# Contribution review, one item at a time

The loop the maintainer wants:

1. Maintainer: **"next item"** (or a number).
2. Claude: analyze, advise, prepare. Ends with a clear recommendation and the
   exact action ready to run.
3. Maintainer reviews, possibly pushes back.
4. Maintainer: **"go"** (or an adjusted instruction). Claude executes, records,
   and says what happened.
5. Repeat.

Claude's eyes are the point. Scripts do the mechanical work so Claude's context
holds the judgment, not the build logs. Everything runs serially: one worktree,
one shared cargo target dir in `~/.local/state/ga-review/`. A parallel pass once
cost 47 GB and 13 concurrent rustc jobs. Never spawn reviewer agents in parallel;
a single subagent per item is fine when the diff is big.

Contributor plumbing: tachsin bases every PR on current main and links it with
"Fixes #N" in the PR body (not in the commit message). He has been quiet since
late Sept 2026, so nothing comes back from him: changes, conflicts and the PRs
he was expected to open for answered issues are finished here, see "Finishing
it ourselves". An active contributor still rebases their own PR; then a
conflicting PR is skipped and comes back mergeable on a later item.

## Scripts (Claude runs these, the maintainer never has to)

| script | does |
|---|---|
| `scripts/queue.sh` | open PRs with size, mergeable state, linked issue, ledger verdict; issues without a PR |
| `scripts/review-pr.sh N` | fetch PR, prove the bug by running its new tests against main's source, then suite, clippy, fmt, examples on the PR branch, conflict check, convention flags; report in `reports/pr-N.md`, worktree left at `wt/` |
| `scripts/ledger.sh N VERDICT "note"` | append to `ledger.tsv`, the memory across sessions |
| `scripts/merge.sh [--dry-run]` | rebase-merge every open PR whose ledger verdict is MERGE, in number order, skipping conflicts; marks MERGED |
| `scripts/review-pr.sh --clean` | drop worktree, pr-* branches, 2 GB target dir; ledger and reports stay |

## "next item"

Run `queue.sh`. The next item is the lowest-numbered open PR or issue with no
final ledger state, taking a PR together with its linked issue. If the maintainer
names a number, take that. Then, by kind:

**PR that fixes something.** Run `review-pr.sh N`. Read the report and the diff
(`git diff origin/main pr-N`). Judge what the script cannot:

- Is the bug confirmed (FAIL or HANG on main source)? If the tests pass on main
  or don't compile there, say so and judge from the code instead.
- Is the fix minimal and in the crate's idiom? Compare with siblings (other
  genotypes, strategies, reporters). Builders report errors through the existing
  `TryFromBuilderError`; new error types or `try_new` constructors for programmer
  misconfiguration are not wanted.
- Does it change results for configs that worked before? Edited seeded
  expectations need a justification in the PR body; unaffected paths must keep
  the same rng stream.
- Supporting machinery: new pub items, new fields, widened signatures. Private
  helpers stay private. `//` on private items, `///` only on public API.
- Anything outside scope: Cargo.lock regeneration, CI, CHANGELOG (the maintainer
  writes it at release), dependency changes.
- Does the issue text match the code it cites?

**PR that changes behaviour or adds surface** (seeded results change, new API,
new platform, CI policy). Run the script anyway, then present the tradeoff
plainly and let the maintainer decide before recommending.

**Docs or CI PR.** Verify every claim against the code. No build needed unless
it touches Cargo.toml or CI.

**Issue with no PR.** Summarize the question, the options the contributor
offered, and recommend one with the reason. Prepare the reply comment text.
When the ledger already holds the answer (ANSWERED) and the author is quiet, the
item is the implementation: restate the decided option and what it will touch,
and implement it on the go.

## Advise and prepare

End every item with, in this order:

1. Verdict: MERGE, CHANGES (name them), REJECT, or DISCUSS, with the one reason
   that decides it.
2. Findings worth the maintainer's attention, few and concrete, with file:line.
3. The GitHub link, so the maintainer can eyeball the code: for a PR
   `https://github.com/basvanwesting/genetic-algorithm/pull/N/files`, for an
   issue `.../issues/N`. On its own line, directly above the prepared action.
4. The prepared action, verbatim: the merge command, or the comment text to post
   on the PR or issue (change request, rejection with reasons, answer to a
   question). Comments are written in the maintainer's voice, short, factual,
   no fluff, since the maintainer reads and approves them before they go out.

When there is anything to change, however small, and the author is active, the
prepared action is the change request, not a merge: merging and then patching
over a contributor's work hides the review from them and from the history. When
the author has gone quiet for weeks (tachsin today), the prepared action is to
finish it ourselves: name the changes we will make on top of their commit.

Then stop and wait. Do not merge, comment, or close anything before the go.

## Finishing it ourselves

For a quiet author, after the go (a change the maintainer already requested on
the PR needs no second go):

1. `git fetch origin pull/N/head:pr-N`, then `git cherry-pick` the contributor's
   commits onto local `main`, so they keep authorship. Resolve conflicts here.
2. Apply the changes as a separate follow-up commit. Its body starts with
   "Follow-up to #N, applied by maintainer as the contributor went quiet." and
   carries what the review found but the commit does not fix.
3. Focused tests, clippy, fmt, and one fresh-eyes review subagent on
   `git diff HEAD~1` (their commit and the follow-up together) before committing.
   Its findings on the contributor's own commit count too: fix them in the
   follow-up, or bring a behaviour change to the maintainer.
4. One item at a time on the single worktree. Run the full `cargo test` once
   after the batch. Never push; the maintainer pushes.
5. After the push, on a go: a cherry-pick does not mark the PR merged and the
   commit messages carry no "Fixes #N", so close both by hand.
   `gh pr close N --comment` naming the contributor's commit and the follow-up,
   `gh issue close M --comment` naming the same, then `ledger.sh N MERGED`.

An answered issue without a PR is implemented the same way from step 2, as a
normal commit whose message ends with "Fixes #M".

## "go"

Execute exactly what was prepared, adjusted by anything the maintainer said:
`gh pr merge N --rebase`, or `gh pr comment` / `gh issue comment` / `gh issue
close`. Record with `ledger.sh` (MERGED, CHANGES, REJECT, CLOSED, ANSWERED).
Report the outcome in two lines and offer nothing; the maintainer says "next"
when ready. After a run of merges, remind about `git pull`, `cargo test` and the
CHANGELOG once, not per item.

A CHANGES PR comes back as an item when the contributor pushes: the queue shows
a new head, `review-pr.sh` runs on it, verdict flips. One that stays untouched
for weeks is finished here instead.

## Learned conventions (Sept 2026, batches 1 to 3)

- Linear history, always rebase merge, contributor keeps authorship.
- Real bugs so far: builder and hot-path panics at edge configs, HashMap
  iteration breaking seeded determinism, a trait method siblings had and one
  lacked, stale docs after deliberate behaviour changes. Every one was confirmed
  by its test failing against main's source.
- Rejected: PR #10, a dependency swap that broke seeded determinism, lost exact
  counts, and regenerated the lockfile; motivated by the contributor's own crate.
- Deferred by policy: anything breaking (`RangeAllele` bounds, `Cache` fields)
  waits for a minor bump. Operator constructors keep panicking on programmer
  misconfiguration.
- The contributor asks good questions in issue bodies. Answer them; an active
  contributor acts on the answer.
- Oct 2026: tachsin went quiet with five PRs on CHANGES; all five landed by
  cherry-pick plus follow-up. The fresh-eyes review of an already approved PR
  still found a behaviour change (#44 moved seeded float RangeScaled results by
  1 ulp), so review the contributor's commit again, not just the follow-up.
