# n - The Funky Package Manager Executor

**Disclaimer:**
Everything in this repo is written by chat gpt except this disclaimer :)

---

Welcome to `n`, the coolest, slickest, and most convenient command-line tool for all your package management needs! Whether you're juggling `npm`, `yarn`, `pnpm`, or `bun`, `n` has got your back. It's like your personal DJ, mixing and matching commands for the right package manager. 🎧🚀

## Getting Started

🔧 **Installation**

Clone this repository and feel the magic:

```
git clone https://github.com/luismeyer/n.git
cd n
cargo build --release
```

Now, move the compiled binary to a location in your PATH. On Unix-like systems, you might do something like:

```
sudo cp target/release/n /usr/local/bin
```

🚀 **Usage**

Run `n` followed by any package manager command you usually use. `n` will automatically detect your project's package manager and forward the command. It's like saying "Abracadabra", but for code!

```
n install
n start
n test
```

✨ **Examples**

- In a directory with `package-lock.json` (npm):

  ```
  n install axios
  ```

- In a directory with `yarn.lock` (yarn):

  ```
  n add lodash
  ```

## Features

- 🕵️‍♂️ Automatic Detection: Identifies which package manager your project uses.
- 🏎️ Fast and Furious: Executes commands quicker than you can say "Fast".
- 🤹‍♂️ Multi-Command Handling: Pass multiple arguments and watch `n` handle them all.
- 💃 Funky and Friendly: Because who said command line tools can't have a personality?

## Contributing

Wanna add some more funk? Suggestions and pull requests are more than welcome. Let's make `n` the funkiest tool out there!

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments

- Hat tip to all the package managers out there, making our lives easier.
- A big shoutout to Rust 🦀, the language that powers `n`.

## Keep it Funky! 🕺

Remember, coding is supposed to be fun. Keep it light, keep it funky, and let `n` handle the mundane stuff.
