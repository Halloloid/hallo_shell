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

## The REPL Loop
it's present in the main.rs so it checkes if the command given matches any of the builtin commands present in the Built-ins commands else it goes to the executor.rs were those commands are executed with the help of `std::process::Command::new(<command>)` 

i had made it a infinte loop which runs until the Built-ins command `exit` is typed we can say that it is the entry point for the commands to be distinguised with the help of **match** if its a Built-ins command then which Built-ins command it is else then its a external command

for checking its which Built-ins command i had used direct command name if it does not need any arguments else i check it with `.starts_with()`

the hardest part may sound funny but it was for me to differnciate if there is pipeline or else Built-ins command its else if its external command

## Tokenizer

It's Present in the parser.rs it basically help convert the whole command in to vec of argument so i can use them as the arg for the command so its supported or the external and as well as builtin command such as for the `echo` for supporting the Quoteing inside the echo 

So i had two function for tokenization one is **split_by_args** for spliting the args which is not quoted and another is **split_by_args_quote** which help's to split the args inside quote as inside quote there is different rules of tokenization and without quotes there different rules of tokenization 

So what i did in **split_by_args** is i convert the command into characters iterabel to see the upcoming character then i perform the action according to ex if the character is `\` so i check if the next character is going to be a `\`,`n`,`'`,`"` so then i see they are esacpe sequence and work and written logic accordin to it this same approach is also used in **split_by_args_quotes** but in a little different way

