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
   │  ├─ handel_declare.rs
   │  ├─ handel_echo.rs
   │  ├─ handel_hallo.rs
   │  ├─ handel_history.rs
   │  ├─ handel_jobs.rs
   │  ├─ handel_pwd.rs
   │  └─ handel_type.rs
   ├─ commands.rs        # registry / dispatch for built-ins
   ├─ parser.rs          # tokenizer / AST for a single line
   ├─ executor.rs        # executes parsed commands (builtin + external)
   ├─ expansion.rs       # for handeling variables which are shell varaiables
   ├─ pipelines.rs       # handels the pipelines present in the command
   ├─ completion.rs      # completion engine & registration
   ├─ redirect.rs        # stdout/stderr redirection helpers
   ├─ shell_helper.rs    # prompt, env helpers, PATH lookup
   └─ logo/
      └─ logo.jpg
```

## Architecture Overview

it's a breif of overview of how the shell works

```
stdin -> rustyline -> REPL Loop -> if builtin -> commands
                                      | 
                                    else -> executor
```
further they are checked if backgroun job, contains pipelines, output redirection and much more

## REPL Loop
it's present in the main.rs 