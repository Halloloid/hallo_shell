# This is _**halloShell**_

![screenshot](https://github.com/user-attachments/assets/73bd6c02-fd9f-4535-aecc-b5fe6cc69345)

A minimal shell implemented in Rust.

Right now this minimal shell has:

---

## Built-in commands

```
hallo
cd
echo
pwd
type
complete
exit
```

## Features

**Core**

- Navigation — `cd`, `pwd`
- Quoting / escaping — `echo` supports single & double quotes and handles backslashes (escape sequences)
- `hallo` — display a simple banner
- `type` — check whether a command is built-in
- `complete` — register or inspect custom completions
- `exit` — quit the shell

**I/O redirection**

- `>` / `1>`  — stdout (truncate)
- `>>` / `1>>` — stdout (append)
- `2>`        — stderr (truncate)
- `2>>`       — stderr (append)

> Tip: use `>` to overwrite, `>>` to append

**Completion**

This shell provides flexible completion for commands and filenames, plus a way to register custom completion scripts.

- Command completion
  - `Tab` — complete the current partial command when a single match exists
  - `Tab` + `Tab` — list matching commands for the current partial input
  - Matches include built-in commands and external commands

- Filename completion
  - `Tab` — complete file or directory names for the current argument
  - Supports nested paths and multiple arguments
  - `Tab` + `Tab` — show matching filenames for the current argument
  - A completed directory may receive a trailing `/` to indicate it's a directory

- Custom completion (the `complete` built-in)
  - Register: `complete -C <script-path> <command>` — attach a completion script to `<command>`
  - Inspect: `complete -p <command>` — show the registered completion for `<command>`
  - Unregister: `complete -r <command>` — remove a registered completion
  - After registration, completions for the command will use the provided script

- Behavior & tips
  - Completion follows the filesystem and command name casing (case-sensitive on case-sensitive systems)
  - Use `Tab` to save typing and `Tab`+`Tab` to explore options
  - if there is common prefix in the command or for file/dirs completeion then the longest common prefix is used



---

## _More Features are Coming Soon_
