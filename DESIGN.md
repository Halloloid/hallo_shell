# App structure

```
halloShell/
├─ Cargo.toml
├─ Cargo.lock
├─ README.md
├─ DESIGN.md
├─ your_program.sh
└─ src/
   ├─ main.rs            # entrypoint: REPL loop
   ├─ lib.rs
   ├─ commands/          # built-in command handlers (one file per command)
   │  ├─ handel_cd.rs
   │  ├─ handel_complete.rs
   │  ├─ handel_echo.rs
   │  ├─ handel_hallo.rs
   │  ├─ handel_jobs.rs
   │  ├─ handel_pwd.rs
   │  └─ handel_type.rs
   ├─ commands.rs        # registry / dispatch for built-ins
   ├─ parser.rs          # tokenizer / AST for a single line
   ├─ executor.rs        # executes parsed commands (builtin + external)
   ├─ completion.rs      # completion engine & registration
   ├─ redirect.rs        # stdout/stderr redirection helpers
   ├─ shell_helper.rs    # prompt, env helpers, PATH lookup
   └─ logo/
      └─ logo.jpg
```

# Comming Soon
- PipeLines
- History
- History Persistance
- Parameter Expansion