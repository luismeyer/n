# n - The Funky Package Manager Executor

**Disclaimer:**
Everything in this repo is written by chat gpt except this disclaimer :)

---

Welcome to `n`, the coolest, slickest, and most convenient command-line tool for all your package management needs! Whether you're juggling `npm`, `yarn`, `pnpm`, or `bun`, `n` has got your back. It's like your personal DJ, mixing and matching commands for the right package manager. 🎧🚀

## Getting Started

🔧 **Installation**

Clone this repository and feel the magic:

```bash
git clone https://github.com/luismeyer/n.git
cd n
cargo build --release
```

Now, move the compiled binary to a location in your PATH. On Unix-like systems, you might do something like:

```bash
sudo cp target/release/n /usr/local/bin
```

🚀 **Usage**

Run `n` followed by any package manager command you usually use. `n` will automatically detect your project's package manager and forward the command. It's like saying "Abracadabra", but for code!

```bash
n install
n start
n test
```

✨ **Examples**

- In a directory with `package-lock.json` (npm):

  ```bash
  n install axios    # Full command
  n i axios         # Using shortcut
  n d               # npm run dev
  ```

- In a directory with `yarn.lock` (yarn):

  ```bash
  n add lodash      # Full command
  n a lodash        # Using shortcut
  n d               # yarn dev
  ```

- In a directory with `pnpm-lock.yaml` (pnpm):

  ```bash
  n i react typescript --save-dev    # pnpm install react typescript --save-dev
  n r lodash                         # pnpm remove lodash
  n b                                # pnpm run build
  ```

## Command Shortcuts ⚡

`n` includes smart command patching that automatically expands common shortcuts to their full commands, tailored for each package manager:

### Universal Shortcuts

These work across all package managers:

```bash
n i           # → install
n a           # → add (or install for npm)
n r           # → remove/uninstall
n rm          # → remove/uninstall
n d           # → dev (run dev for npm/pnpm/bun)
n b           # → build (run build for npm/pnpm/bun)
n s           # → start
n t           # → test
n up          # → update/upgrade
n ls          # → list
```

### Package Manager Specific Behavior

- **npm**: `n a` becomes `npm install` (since npm doesn't have an `add` command)
- **yarn**: `n d` becomes `yarn dev` (direct script execution)
- **npm/pnpm/bun**: `n d` becomes `[manager] run dev` (requires `run` prefix)

### Examples with Shortcuts

```bash
# Instead of typing:
npm install lodash --save-dev

# Just type:
n i lodash --save-dev

# Or run development server:
n d              # Expands to appropriate dev command for your package manager
```

## Default Fallback 🎯

When `n` doesn't detect any package manager lock files in your current directory (or up to 5 parent directories), it provides an interactive fallback:

### Interactive Selection

```bash
$ n install react
No package manager detected. Please select one:
❯ pnpm
  bun  
  npm
  yarn
```

### Smart Initialization

- **For install commands** (`n i`, `n install`, `n add`, `n a`): Runs the command directly to initialize and install packages
- **For other commands**: First runs `install` to initialize the project, then executes your original command

### Example Workflow

```bash
# In a fresh directory without lock files:
$ n start
No package manager detected. Please select one:
❯ pnpm

Selected: pnpm
Initializing project with pnpm...
[pnpm install runs]
Running original command...
[pnpm start runs]
```

## Features

- 🕵️‍♂️ **Automatic Detection**: Identifies which package manager your project uses by scanning for lock files up to 5 parent directories.
- 🏎️ **Fast and Furious**: Executes commands quicker than you can say "Fast".
- 🤹‍♂️ **Multi-Command Handling**: Pass multiple arguments and watch `n` handle them all.
- ⚡ **Command Patching**: Smart shortcuts that expand common abbreviations to full commands (e.g., `n i` → `npm install`, `n d` → `yarn dev`).
- 🎯 **Default Fallback**: Interactive package manager selection when no lock file is detected, with intelligent project initialization.
- 💃 **Funky and Friendly**: Because who said command line tools can't have a personality?

## Contributing

Wanna add some more funk? Suggestions and pull requests are more than welcome. Let's make `n` the funkiest tool out there!

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- Hat tip to all the package managers out there, making our lives easier.
- A big shoutout to Rust 🦀, the language that powers `n`.

## Keep it Funky! 🕺

Remember, coding is supposed to be fun. Keep it light, keep it funky, and let `n` handle the mundane stuff.
