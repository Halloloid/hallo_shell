# This is _**halloShell**_

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

### Core

- Navigation: **`cd`**, **`pwd`**
- Quoting: **`echo`** supports single and double quotes and handles backslashes (escape sequences)

### I/O redirection

- Redirect stdout and stderr using familiar operators:
  - `>` / `1>`  — redirect **stdout** (truncate)
  - `2>`        — redirect **stderr** (truncate)
  - `>>` / `1>>`— append **stdout**
  - `2>>`       — append **stderr**

> Tip: `>` vs `>>` controls whether output overwrites or appends.

### Auto-completion

- `Tab`       — partial command completion
- `Tab` + `Tab` — show matching commands for the current partial input

---

## _More Features are Coming Soon_
