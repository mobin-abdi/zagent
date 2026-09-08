Add a new `shell` tool to the existing ZAgent codebase.

Before making changes:
- Inspect the existing `Tool` trait.
- Inspect `ToolRegistry`.
- Inspect one or two existing tools such as `file_read` and `file_write`.
- Follow the same architecture and coding style.

Implementation:
1. Create `src/tools/shell.rs`.
2. Define a `Shell` tool implementing the existing `Tool` trait.
3. The tool must accept a command string, for example:
   `{"command": "pwd"}`
4. Execute the command using Rust's standard library:
   `std::process::Command`
5. Return:
   - stdout
   - stderr
   - exit code
6. Register the new tool in the existing `ToolRegistry`.
7. Make sure the tool is included in the tools exposed to the LLM.
8. Do not add any new dependencies.
9. Do not modify unrelated tools or the Agent Loop.

After implementation:
- Run `cargo check`.
- Fix any compilation errors caused by your changes.
- Report which files you changed and whether `cargo check` passed.

Do not implement permissions, confirmation, or security restrictions yet. This task is only about adding the basic shell tool. 