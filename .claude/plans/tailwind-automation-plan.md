# Plan: Automate Tailwind CSS Compilation

## Problem
Currently requires manual `npm run build` before `trunk serve --open`, which is easy to forget.

## Evaluation Summary

### Option 1: Trunk Pre-Build Hook ⭐ RECOMMENDED
**Approach:** Create `Trunk.toml` with a pre-build hook that runs `npm run build` automatically before every trunk command.

**Why This Wins:**
- Zero workflow change - developers continue using `trunk serve --open`
- No CI changes needed - existing `trunk build --release` will automatically compile CSS
- Proper semantics - Trunk hooks are designed for exactly this use case
- Fail-fast - build stops immediately if CSS compilation fails
- Cross-platform compatible
- 4 lines of config, 18ms overhead (negligible)

### Option 2: build.rs ❌ NOT RECOMMENDED
**User's Question:** "I think there is an option to use a build.rs file but I'm not sure how that will work."

**Analysis:** While technically possible, build.rs is **architecturally wrong** for this use case:
- `build.rs` is for generating Rust code (proc macros, FFI bindings), not compiling independent web assets
- Imperfect change detection - can't reliably track which `.rs` files use Tailwind classes
- Must choose between: tracking entire `src/` (rebuilds CSS on ANY Rust change) or not tracking it (misses new CSS classes)
- Critical flaw: Trunk doesn't trigger cargo rebuild when `styles/main.css` changes - you'd still need a separate watch process
- Hidden output unless `cargo build -vv`
- Misleading - suggests Rust compilation depends on CSS when it doesn't

**Verdict:** Don't use build.rs for this. It's the wrong tool, and you'd need workarounds that defeat its purpose.

### Option 3: Parallel Processes (Manual Fallback)
Run `npm run watch` and `trunk serve --open` in separate terminals. Works but requires remembering two commands (same problem as before) and needs CI changes.

### Option 4: Shell Script Wrapper
Create `scripts/serve.sh` that runs npm then trunk. Requires training developers to use script instead of trunk directly, and Windows compatibility issues.

## Recommended Implementation

### Step 1: Create Trunk.toml
**File:** `/home/steve/Projects/yew-countdown-solver/Trunk.toml` (new file)

```toml
[[hooks]]
stage = "pre_build"
command = "npm"
command_arguments = ["run", "build"]
```

### Step 2: Update Documentation (Optional)
**File:** `/home/steve/Projects/yew-countdown-solver/CLAUDE.md`

Add note under "Build & Development Commands" section:
```markdown
Note: CSS is automatically compiled via Trunk pre-build hook.
The `npm run build` command runs automatically before each trunk build.
```

### Step 3: Consider Untracking output.css (Optional)
Currently `styles/output.css` is committed to git, which is why CI works without npm. Options:

**Option A: Keep committing output.css** (safest)
- Pro: Works in environments without Node.js
- Pro: Faster first build after clone
- Pro: Zero CI changes
- Con: Git diffs include generated CSS

**Option B: Make output.css build-time only**
- Add `styles/output.css` to `.gitignore`
- Update CI to run `npm install` before trunk build (line 21 in `.github/workflows/main.yml`)
- Git commit the removal of output.css

Recommendation: **Keep Option A** unless you have strong preference for not committing generated files.

## Critical Files

- `/home/steve/Projects/yew-countdown-solver/Trunk.toml` - **CREATE** with pre-build hook
- `/home/steve/Projects/yew-countdown-solver/CLAUDE.md` - **UPDATE** documentation (optional)
- `/home/steve/Projects/yew-countdown-solver/.gitignore` - **UPDATE** if choosing Option B
- `/home/steve/Projects/yew-countdown-solver/.github/workflows/main.yml` - **UPDATE** if choosing Option B (add npm install step)
- `/home/steve/Projects/yew-countdown-solver/package.json` - No changes needed
- `/home/steve/Projects/yew-countdown-solver/styles/output.css` - Optionally untrack from git

## Verification

### Development Workflow Test
```bash
# From clean state, run trunk serve
trunk serve --open

# Expected output should show:
# 1. "≈ tailwindcss v4.1.18" message from pre-build hook
# 2. "Compiling yew-countdown-solver..." from Rust build
# 3. Browser opens with styles working correctly
```

### Production Build Test
```bash
trunk build --release

# Expected: Same CSS compilation message before WASM build
# Verify: dist/output-{hash}.css exists with correct styles
```

### CSS Change Propagation Test
```bash
# Start trunk serve
trunk serve --open

# In another terminal, modify a Rust file to add new Tailwind class
echo '// test' >> src/app.rs

# Expected: Trunk detects change, reruns pre-build hook, recompiles CSS and WASM
# Browser auto-reloads with new styles
```

### CI Test
```bash
# Push code with Trunk.toml to GitHub
git add Trunk.toml CLAUDE.md
git commit -m "Add Trunk pre-build hook for automatic CSS compilation"
git push

# Expected: CI passes with trunk build --release working as before
# Check Actions tab: should see npm output in build logs
```

### Error Handling Test
```bash
# Break the CSS to test fail-fast behavior
echo 'invalid css syntax' >> styles/main.css
trunk build

# Expected: Build fails immediately with clear npm error
# Expected: Rust compilation never starts
```

## Alternative Approaches Documentation

If you prefer **parallel processes** approach instead (for independent CSS/WASM builds):

1. Development: Run these in separate terminals:
   ```bash
   npm run watch        # Terminal 1 - watches and rebuilds CSS
   trunk serve --open   # Terminal 2 - watches and rebuilds WASM
   ```

2. Production: Run before trunk:
   ```bash
   npm run build && trunk build --release
   ```

3. CI: Add npm install and build to workflow:
   ```yaml
   - run: |
       npm install
       npm run build
   - run: trunk build --release
   ```

This gives you independent control but requires remembering multiple commands.

## Summary

**Answer to your question:** Yes, `build.rs` is technically possible, but it's the wrong tool architecturally. Build scripts are for Rust code generation, not web asset compilation. The change detection doesn't work properly with Trunk's file watching.

**Best solution:** Trunk pre-build hook. 4 lines of config, zero workflow changes, works everywhere, solves your problem completely.

**Implementation:** Just create the `Trunk.toml` file above and you're done. Everything else works automatically.
