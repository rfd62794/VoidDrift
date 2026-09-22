# VoidDrift has a prose roadmap with no roadmap block

## 1. Why this exists

This directive was generated from a workspace scan, not written by hand. It matched the
`missing-roadmap` category in `Portfolio/backlog_policy.yaml`, which Robert authorised for
automatic dispatch:

> Robert asked for a roadmap in every repo so the swarm can progress each one on its own. The run only writes docs/DIRECTION.md (when missing) and docs/ROADMAP.md with status `draft`; nothing moves until Robert approves the roadmap, so drafting it is safe to authorise once for all of them.

Nobody looked at this specific case before it was dispatched. Treat the finding as a
claim to verify, not as an instruction - §5 says what to do if it is wrong.

## 2. Scope

```
docs/ROADMAP.md
docs/DIRECTION.md
```

At most 2 file(s). A change that needs more than that is a
different task: stop and report it rather than widening this one.

## 3. The work

This repo has no usable `yaml roadmap` block in `docs/ROADMAP.md`, so the swarm
has nothing to propel against. Draft one - and draft `docs/DIRECTION.md` first,
when that is missing too.

1. Read the repo's own evidence and nothing else: the README, `AGENTS.md` /
   `CLAUDE.md`, `docs/` (recent `docs/directives/` especially), and
   `git log --oneline -40`. Nothing outside the repo.
2. If `docs/DIRECTION.md` is missing, write it with the sections Purpose /
   Current state / Next steps / Definition of done / Do not / Sources of truth.
   Every claim comes from the repo's own evidence, with the source named.
3. Write `docs/ROADMAP.md` in the exact format below. **If the file already
   exists, keep ALL of its prose** - add the `yaml roadmap` block (and a short
   "Why these milestones" paragraph if there isn't one), and never delete
   existing text.
   - `status: draft`, `reviewed:` today's date, `stop_if:` one sentence - the
     condition that kills or parks this roadmap.
   - 2 to 4 milestones, the first `active`, each with 1-3 **checkable** exit
     entries. A `test` entry uses the repo's documented test command and
     runs in the repo root.
   - 2-5 steps per milestone. **Every** step gets `size`, `value`, and at least
     one `accept` entry (the same `test`/`file`/`grep` shape as a milestone
     `exit` - it is how the swarm proves the step finished). Use `needs` to
     name the step ids that must finish first, where order matters. Size steps
     so one agent run finishes one step: S is under 30 minutes, M is about one
     run, and L means split it unless it is a design step.
   - Milestones come from the repo's own stated next steps and unfinished
     work - no invented features.
4. Change nothing else. Commit the files you wrote. Report the milestones in
   one line each.

The format (the `RFDGameStudio` lines are the spec's example - substitute this
repo's name and its own evidence):

````markdown
# Roadmap: RFDGameStudio

Why these milestones, in plain words.

```yaml roadmap
status: draft            # draft | approved
approved: ""             # "2026-09-22 Robert" once approved
reviewed: "2026-09-22"   # last human or model review - the swarm re-plans when stale
replan_after_days: 14    # reviewed older than this -> stale (default 14)
stop_if: "One sentence - the condition that kills or parks this roadmap."
revive_if: ""            # parked repos only: what would revive it
milestones:
  - id: M1
    title: Every published game has a way back to the arcade
    status: active       # pending | active | done | blocked
    exit:                # all must hold for the milestone to be done
      - test: "cd ts && npx vitest run"          # command exits 0 (run in the repo root)
      - file: "ts/src/ui/GameShell.tsx"         # path exists
      - grep: {path: "data/arcade.json", pattern: "\"exit\""}   # regex found in file
    steps:
      - id: M1.1
        title: Adopt GameShell in the three remaining games
        kind: refactor   # tests | docs | refactor | fix | feature | design
        size: M          # S < 30 min | M one agent run | L = split it
        value: 4         # 1-5, how much it moves the milestone
        needs: []        # step ids in this roadmap that must be done first
        status: pending  # pending | queued | done
        directive: ""    # filled by the swarm when it creates one
        detail: One paragraph - what, and why it moves the milestone.
        accept:          # at least one - how the swarm proves the step is done
          - test: "cd ts && npx vitest run"
```
````

Rules: ids are stable; at most one milestone `active`; `exit` and `accept`
checks are only `test`, `file`, `grep` (no arbitrary shell beyond `test`, which
runs with a 15-minute timeout in the repo root).

Verify with `git status --porcelain`: it must show only `docs/DIRECTION.md` and
`docs/ROADMAP.md`. Paste the `yaml roadmap` block from your `docs/ROADMAP.md`
into your report.

## Rules for this run

- **Write only inside this worktree.** Never `%TEMP%`, never `/tmp`. Scratch goes in `.devin-scratch/`.
- **Do not delete anything.** `rm` is not permitted in a headless run and ends it silently.
- **One shell command at a time.** No `&&` or `;` chains, no `$(...)`, no heredocs, no `cat`
  piped into a command. Each of these ended a real run without a word.
- Commit with a plain single-line message: `git commit -m "one line"`.
- Use the `python` already on PATH. Do not probe for interpreters or create a venv.
- No servers and no long-running processes.
- Never merge, rebase onto, or push to the default branch.
- **If a command is refused, stop immediately** and report Blocked naming the refused
  command. Working around a refusal is what killed every run that died silently; the
  refusal itself is useful information and reporting it is a successful outcome.
- **If the task turns out to be wrong, stop and say so.** This directive was generated
  automatically from a scan, and a scan can be wrong. Reporting "this was a false
  positive, here is why" is a complete and welcome result - do not invent work to do.
- **This change is purely additive.** Create the new file and change nothing else. If
  making it pass would require editing existing code, stop and report that instead -
  that is a finding worth more than the file.

## 5. If the finding is wrong

The scan that produced this can be wrong: a module may be tested somewhere the scan did
not look, a README may live one level up, a marker may already be resolved. If so, stop,
report Blocked, and state what the scan missed. That report is how the finder gets
fixed, and it is worth more than the work would have been.

## 6. Completion criteria

- [ ] `git status --porcelain` passes.
- [ ] Nothing outside §2 was modified.
- [ ] Nothing was deleted.
- [ ] If anything was refused or the finding was wrong, it is reported rather than worked around.

## 7. Report

What you changed and why. The real output of the check command, pasted. Anything about
the finding that was inaccurate. And any refused command, verbatim.

<!-- check: git status --porcelain -->

<!-- queue:start -->
## Queue

| Field | Value |
|---|---|
| Status | Review |
| Assigned to | devin |
| Branch | directive/voiddrift-auto-missing-roadmap-roadmap |
| Base branch | - |
| Policy | missing-roadmap |

**Status log**
- 2026-09-22 13:07 · backlog-policy · none → Queued — generated from a missing-roadmap finding authorised in backlog_policy.yaml
- 2026-09-22 15:15 · robert-claude · Queued → Approved
- 2026-09-22 15:15 · dispatcher · Approved → In progress — dispatched devin in C:\GitHub\.worktrees\VoidDrift--voiddrift-auto-missing-roadmap-roadmap; base origin/main (local main differs)
- 2026-09-22 15:18 · agentflow-tick · In progress → Blocked — Devin run died on an unapproved confirmation-gated tool call in non-interactive mode before producing any output; redispatch with --permission-mode dangerous.
- 2026-09-22 15:24 · robert-claude · Blocked → Queued — refused command was `ls docs\directives docs\adr tests 2>$null` - a redirect makes the call confirmation-gated. The prompt now forbids pipes and redirects (PR #44). Retrying.
- 2026-09-22 15:24 · robert-claude · Queued → Approved
- 2026-09-22 15:24 · dispatcher · Approved → In progress — dispatched devin in C:\GitHub\.worktrees\VoidDrift--voiddrift-auto-missing-roadmap-roadmap
- 2026-09-22 15:34 · devin · In progress → Review — Created docs/DIRECTION.md and added a yaml roadmap block (4 milestones: M1 launch blockers, M2 structural rework, M3 Phase 4b narrative, M4 economy redesign groundwork) to docs/roadmap.md — all existing prose kept, purely additive. Committed 5a207dc on directive branch; git status --porcelain clean.
<!-- queue:end -->
