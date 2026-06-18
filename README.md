# This is _**halloShell**_
<img width="737" height="390" alt="image" src="https://github.com/user-attachments/assets/73bd6c02-fd9f-4535-aecc-b5fe6cc69345" />

A minimal shell implemented in Rust.

Right now this minimal shell has:

---

## Built-in commands

```
cd
echo
pwd
type
exit
```

## Features

**Core**

- Navigation — `cd`, `pwd`
- Quoting — `echo` supports single and double quotes and handles backslashes (escape sequences)

**I/O redirection**

- `>` / `1>`  — stdout (truncate)
- `2>`        — stderr (truncate)
- `>>` / `1>>`— append stdout
- `2>>`       — append stderr

> Tip: use `>` to overwrite, `>>` to append

**Completion**

- Command completion
  - `Tab` — complete the current partial command when a single match exists
  - `Tab` + `Tab` — list all matching commands for the current partial input
  - Matches include built-in commands and external commands too.

- Filename completion
  - `Tab` — complete file or directory names for the current argument
  - Supports nested paths (e.g., `src/mai` -> `src/main.rs`) and multiple arguments
  - `Tab` + `Tab` — show all matching filenames for the current argument
  

- Behavior 
  - Completion is case-sensitive (matches the filesystem and command names)

---

## _More Features are Coming Soon_
