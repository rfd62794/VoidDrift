# Agent Contract

**Date:** May 2026
**Scope:** Universal rules that apply to ALL agents (Claude, Windsurf, or any future agent)

---

## Proof Standard

**Raw terminal output + screenshots only. Never summaries.**

When reporting results of code execution, testing, or verification:
- Paste the exact terminal output from the command
- Include screenshots if visual verification is required
- Do not summarize or paraphrase the output
- Do not interpret the results — let the user draw conclusions

**Example:**
```
❌ BAD: "The build succeeded"
✅ GOOD: 
PS C:\Github\VoidDrift> cargo check
    Checking voidrift v0.1.0 (C:\Github\VoidDrift)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.34s
```

---

## One Problem Per Directive

**Focus on a single issue. Do not scope creep.**

When given a directive to fix or implement something:
- Address only the specific problem stated
- Do not add "bonus" features or improvements
- Do not fix unrelated issues you notice
- If you see additional problems, report them separately

**Rationale:** Prevents scope drift and ensures the user controls the work scope.

---

## Compile After Every File Change

**Never accumulate uncompiled changes.**

After editing any source file:
- Run `.\verify.ps1` to verify compilation
- If compilation fails, fix it before proceeding
- Do not move to the next file until the current one compiles

**Rationale:** Prevents cascading compilation errors that become harder to debug.

---

## Never Force-Push Dev Without Explicit Confirmation

**Force-pushing dev is destructive.**

Before running any force-push command:
- Stop and ask the user for explicit confirmation
- Explain what will be overwritten
- Wait for "yes" or explicit approval

**Example:**
```
❌ BAD: git push origin dev --force
✅ GOOD: "About to force-push dev. This will overwrite origin/dev with local changes. Confirm?"
```

---

## Never Reset Branch to Tag Without Confirmation

**Resetting to a tag destroys uncommitted work.**

Before running `git reset --hard <tag>`:
- Stop and ask the user for explicit confirmation
- Explain what uncommitted work will be lost
- Wait for "yes" or explicit approval

**Rationale:** Prevents accidental loss of work.

---

## egui::Window is Unreliable

**Use painter + ui.interact() pattern instead.**

When implementing UI buttons or interactive elements:
- Do not use `egui::Window` for buttons or click detection
- Use `painter` to draw the button/element
- Use `ui.interact(btn_rect, id, Sense::click())` for click detection
- This pattern is proven to work on bevy_egui 0.33.0

**Rationale:** bevy_egui 0.33.0 doesn't properly handle click events on egui::Window buttons. The painter + ui.interact() pattern is the working solution.

**Example:**
```rust
// ✅ GOOD: painter + ui.interact()
let btn_rect = ui.painter().add(rect);
if ui.interact(btn_rect, id, Sense::click()).clicked() {
    // handle click
}

// ❌ BAD: egui::Window
egui::Window::new("Button").show(ui.ctx(), |ui| {
    if ui.button("Click me").clicked() {
        // this may not work
    }
});
```

---

## Viewport: Multiply egui Logical Coordinates by EGUI_SCALE

**EGUI_SCALE = 3.0 applies to world coordinates, not egui panels.**

When converting between egui logical coordinates and world space:
- Multiply egui logical coordinates by `EGUI_SCALE` (3.0) only
- Do not apply EGUI_SCALE to egui panel dimensions (ui.available_height(), etc.)
- EGUI_SCALE applies to Bevy world coordinates, not egui UI coordinates

**Rationale:** EGUI_SCALE is a Bevy world coordinate multiplier. egui panels use their own logical coordinate system that does not need scaling.

**Example:**
```rust
// ✅ GOOD: world coordinates get scaled
let world_pos = egui_pos * EGUI_SCALE;

// ❌ BAD: panel dimensions do not get scaled
let scaled_height = ui.available_height() * EGUI_SCALE; // wrong
```

---

## Agent Going in Circles = Stop, Demand One Diagnostic Step

**If you're stuck in a loop, stop and ask for diagnostic information.**

When you find yourself:
- Repeating the same action without progress
- Making the same change multiple times
- Unable to identify the root cause

**Stop immediately and:**
- Report the current state
- Ask for one specific diagnostic step
- Do not continue trying fixes blindly

**Example:**
```
❌ BAD: Try fix A → fails → try fix B → fails → try fix C → fails
✅ GOOD: "I've tried X, Y, Z without success. Can you run this diagnostic command and share the output?"
```

**Rationale:** Prevents infinite loops and wasted time. Diagnostic information from the actual environment is more valuable than guessing.

---

## Read Before Touching

**Always read the relevant directive fully before touching any file.**

Before implementing any change:
- Read the directive or issue description completely
- Understand the full scope and requirements
- Do not make assumptions based on partial information

**Rationale:** Prevents implementing the wrong thing or missing requirements.

---

## Report Findings Before Implementing

**Investigate first, report findings, then implement.**

When given a task to fix or implement something:
1. Investigate the current state
2. Read relevant files
3. Report what you found
4. Wait for confirmation before making changes

**Rationale:** Ensures the agent and user are aligned on the problem before making changes.

---

## No Acknowledgment Phrases

**Do not use phrases like "You're absolutely right" or "Great idea".**

- Jump straight into addressing the request
- Do not validate or express agreement with the user's statement
- Do not acknowledge or validate the request before addressing it

**Rationale:** Saves tokens and keeps responses focused on the work.

---

## Context Reconstruction

**Every session starts with context reconstruction.**

At the start of each session:
- Read relevant documentation (AGENT_CONTRACT.md, WINDSURF.md, DEVELOPMENT_PIPELINE.md)
- Check the current state of the codebase
- Review recent commits or changes
- Do not assume knowledge from previous sessions

**Rationale:** Agents do not have persistent memory across sessions. Each session must reconstruct context.

---

## Forbidden Actions

**Never do these without explicit user permission:**

- Delete files
- Reset branches
- Force-push
- Run destructive commands (e.g., `cargo clean`, `rm -rf`)
- Install system dependencies
- Make external requests (API calls, web requests)
- Modify git history
- Change branch structure

**Rationale:** These actions are destructive or have side effects that the user must explicitly authorize.

---

## Verification Tools

**Prefer automated verification when available.**

When verifying work:
- Use automated tests if available
- Use cargo check/cargo test for compilation verification
- Use the local itch preview tool for WASM verification
- Provide copy-pastable commands for manual verification if tools are unavailable

**Rationale:** Automated verification is more reliable and repeatable than manual checks.

---

## Maximize Parallel Tool Calls

**Execute independent operations in parallel whenever possible.**

When making multiple tool calls:
- If operations are independent, execute them simultaneously
- Batch independent actions together
- Only use sequential calls when output of A is required for input of B

**Rationale:** Parallel execution is 3-5x faster than sequential calls.

---

## Minimal Focused Edits

**Use the edit or multi_edit tools for code changes.**

- Prefer minimal, focused edits
- Keep changes scoped
- Follow existing style
- Write general-purpose solutions
- Avoid helper scripts or hard-coded shortcuts

**Rationale:** Minimal edits are easier to review and less likely to introduce bugs.

---

## Immediate Runnability

**Generated code must be immediately runnable.**

When writing new code:
- Add all necessary import statements
- Include all required dependencies
- Ensure code compiles and runs
- Test before delivering

**Rationale:** Prevents交付 broken code that requires additional work to make functional.

---

## Imports at Top of File

**Imports must always be at the top of the file.**

When editing files:
- If adding imports, add them at the top
- Do not add imports in the middle of the file
- If editing code that needs new imports, add imports in a separate edit

**Rationale:** Imports at the top is standard Rust practice and poor code style to place them elsewhere.

---

## Summary on Token Exhaustion

**The system will automatically summarize your work if you run out of tokens.**

- Do not worry about running out of tokens
- Work as if you have infinite token context
- Do not try to take shortcuts to save tokens
- The system will handle summarization automatically

**Rationale:** Prevents agents from making poor decisions due to token pressure.

---

## Persistence

**Only stop for true issues that require user aid.**

- Be persistent in solving problems
- Do not give up easily
- Adjust your plan as needed and sensible
- Avoid looping on the same action if it hasn't worked

**Rationale:** Many problems require multiple attempts and iterations to solve.

---

## Related Documentation

- `docs/WINDSURF.md` — Windsurf-specific context
- `docs/DEVELOPMENT_PIPELINE.md` — Development workflow
- `docs/ARCHITECTURE.md` — Codebase architecture
- `docs/DEVELOPER.md` — Developer onboarding
