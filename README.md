# ZAgent

> A lightweight, fast and extensible AI agent written in Rust.

ZAgent is an open-source AI agent built with **Rust**, designed to be lightweight, extensible, and practical.

It combines an LLM-driven agent loop with a modular tool system, allowing the agent to interact with files, perform calculations, inspect directories, and execute tasks through tools.

The project is currently under active development.

---

## ✨ Features

* 🦀 Written in Rust
* 🤖 LLM-powered agent loop
* 🛠️ Modular tool-calling system
* 📁 File reading
* 📝 File writing
* 📂 Directory listing
* 🧮 Calculator tool
* ⚡ Lightweight runtime
* 🧩 Extensible tool registry
* 💻 Linux support
* 🪟 Windows support
* 🔌 Designed for future integrations and tools

### Current tools

| Tool             | Description                       |
| ---------------- | --------------------------------- |
| `file_read`      | Read files from the filesystem    |
| `file_write`     | Create or modify files            |
| `list_directory` | Inspect directories               |
| `calculator`     | Evaluate mathematical expressions |
| `shell`          | Run shell commands                |

More tools will be added as the project evolves.

---

## 🏗️ Architecture

ZAgent follows a modular architecture built around an agent loop and a tool registry.

```text
                ┌──────────────┐
                │     User     │
                └──────┬───────┘
                       │
                       ▼
                ┌──────────────┐
                │   Agent Loop │
                └──────┬───────┘
                       │
                       ▼
                ┌──────────────┐
                │     LLM      │
                └──────┬───────┘
                       │
                 Tool Calling
                       │
                       ▼
                ┌──────────────┐
                │ Tool Registry│
                └──────┬───────┘
                       │
        ┌──────────────┼──────────────┐
        ▼              ▼              ▼
   File Read      File Write    Directory List
                                       │
                                       ▼
                                  Calculator
```

The tool system is intentionally modular so new capabilities can be added without tightly coupling them to the core agent.

---

## 🚀 Getting Started

### Requirements

* Rust
* Cargo
* An LLM API key
* Linux or Windows

Install Rust through the official Rust installation instructions.

Then clone the repository:

```bash
git clone https://github.com/mobin-abdi/zagent.git
cd zagent
```

Build the project:

```bash
cargo build --release
```

Run it:

```bash
cargo run -- --env=path/to/your/env_file
```

## ⚙️ Configuration

ZAgent uses environment variables for configuration.

Create a local environment file:

```bash
cp .env.example .env
```

Then configure the required values.

Example:

```env
LLM_API_KEY=your_api_key
```

> Never commit API keys, tokens, passwords, or other secrets to the repository.

---

## 🧪 Development

Run the test suite:

```bash
cargo test
```

Check formatting:

```bash
cargo fmt --check
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Build the project:

```bash
cargo build
```

For a release build:

```bash
cargo build --release
```

---

## 🛠️ Adding a Tool

ZAgent is designed around a registry-based tool system.

A new tool should implement the project's tool interface and then be registered with the tool registry.

Conceptually:

```rust
tools.register(MyTool);
```

This makes it possible to extend ZAgent without modifying the core agent loop for every new capability.

---

## 📦 Releases

Official releases will provide pre-built binaries for supported platforms.

Planned release artifacts:

```text
Linux x86_64
Linux ARM64

Windows x86_64
Windows ARM64
```

Linux releases will be distributed as `.deb` packages.

Windows releases will be distributed as executable installers.

Checksums will also be provided for release artifacts.

---

## 🗺️ Roadmap

ZAgent is still evolving.

Planned improvements include:

* [ ] More built-in tools
* [ ] Better tool execution
* [ ] Improved error handling
* [ ] Streaming LLM responses
* [ ] Conversation/session management
* [ ] Better configuration system
* [ ] Cross-platform packaging
* [ ] Automated GitHub Releases
* [ ] Linux `.deb` packages
* [ ] Windows installers
* [ ] Release checksums
* [ ] More comprehensive tests
* [ ] Improved documentation
* [ ] Plugin/extension system
* [ ] Better agent planning and execution

The roadmap may change as the project develops.

---

## 🤝 Contributing

Contributions are welcome.

Before submitting a pull request:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Please keep changes focused and avoid unnecessary modifications.

For larger changes, opening an issue first is recommended so the design can be discussed before implementation.

See `CONTRIBUTING.md` for more information.

---

## 🔒 Security

If you discover a security vulnerability, please do not disclose sensitive details publicly in an issue.

Instead, follow the security reporting instructions provided in `SECURITY.md`.

---

## 📄 License

ZAgent is licensed under the **Apache License 2.0**.

See [`LICENSE`](LICENSE) for the full license text.

Copyright © 2026 Mobin Abdi

---

## ⭐ Support the Project

If you find ZAgent useful:

* ⭐ Star the repository
* 🐛 Report bugs
* 💡 Suggest improvements
* 🔧 Contribute code
* 📖 Improve the documentation

Every contribution helps make the project better.

---

## ⚠️ Project Status

ZAgent is currently in early development.

APIs, configuration formats, tool interfaces, and internal architecture may change between releases.

Do not rely on undocumented behavior in production yet.
