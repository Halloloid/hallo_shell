# This is _**halloShell**_

![screenshot](https://github.com/user-attachments/assets/73bd6c02-fd9f-4535-aecc-b5fe6cc69345)

blog :- https://my-hallo-project-blogs.hashnode.dev/wonderd-about-a-shell-in-rust

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
jobs
history
declare
exit
```

## Features

### **Core**

- Navigation — `cd`, `pwd`
- Quoting / escaping — `echo` supports single & double quotes and handles backslashes (escape sequences)
- `hallo` — display a simple banner
- `type` — check whether a command is built-in
- `complete` — register or inspect custom completions
- `exit` — quit the shell
- `jobs`- helps to see all running and done jobs which are in the background
- `history` - see all the command you typed till now from the start of the shell
- `declare` - helps to declare a shell variables

### **I/O redirection**

- `>` / `1>`  — stdout (truncate)
- `>>` / `1>>` — stdout (append)
- `2>`        — stderr (truncate)
- `2>>`       — stderr (append)

> Tip: use `>` to overwrite, `>>` to append

### **Completion**

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

### **Background Jobs**
- if you end a command with `&` (e.g `<command> &`) then that is executed in the background it will generate a job number and a pid and we can still use the shell for other commands
- if you want to see all the commands running in the background you can use the built-in command `jobs` to see
- to check the recent jobs
   - `+` for the recent command that gone to background
   - `-` for the second recent command
   - ` ` space for rest of the commands
- if it has a Status `Running` then its still running in Background else then `Done` means its completed
- if the background job is completed it prints with `Done` before running another command

### **Pipelines**
- it will help to pass the stdout of one command to the stdin of the another command (**e.g** `<command1> | <command2>` command1's stdout will be gone to the stdin for the command2)
- it also support if one of the command is a **BUILT_IN** command's
- it also support multi command pipelines for external command's

### **History**
- if you want to see all the commands you typed since the start of the shell you use the **BUILT-IN** command `history` for viewing
- you can also use flags with a path:
  - `-r` flag: reads the history from the path provided and add it to the history of the shell
  - `-w` flag: writes all the history of the shell in to the path script provided
  - `-a` flag: appends the new history to the path script provided
- there is also support for the arrow key:
   - if pressed `<UPARROW>` it gives the most recent command typed
   - if pressed `<DOWNARROW>` it gives the back recent command typed
- also support for env variable `HISTFILE` if there with the path then it reads the history and add it to the history of the shell on the start
- after hitting exit in the shell the exit is also added to the history of the shell
## 
### **Parameter Expansion**
- you can add your own shell variables with its value with the help of `declare` **BUILT_IN** command
- if you want to check if the shell variables exists or not you can do it with the help of `-p` flag with the shell variable it will print if its there else it well give not present
- it also support expansion so after declaring the variable you can use it other command as `$<Variable>` if the `<Variable>` is declared then it will use the value the `<Variable>` was given
- it also supports expansion as `${<Variable>}` for checking for the more appropriate variable declared
- if the `${<Variable>}` and the `Variable` is not declared then it will treat it as empty variable 
>Note : the shell variable you want to declare must start with a letter or _ and it can contain letters,digits and underscore in the rest of the body
---

