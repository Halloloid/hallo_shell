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

## Executor

It's the thing contains most of the thing and connected most of the project and it's present inside the executor.rs so as the name sounds it's help to execute the external normal commands which are not built-in commands and it also checks if a certain env variabel is present in env or not and also if its executabel or not so it also has redirection for the stdout and stderr to the files 

It have only one function and i had pretty much added every thing inside not got time to optimize it but its working so now the approach

so in the function there arguments that are passed are background jobs if there is redirection if redirection then is stdout or stderr and inside it is if apppend or write or is have & so it is a background jobs and then prefix of the command is extracted and other is passed as a argument to and output is printed in terminal or inside a file if there is redirection and in aslo checks if the command present in env and if its execuatbel or not

## Built-ins
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

- Navigation — `cd`, `pwd`
- Quoting / escaping — `echo` supports single & double quotes and handles backslashes (escape sequences)
- `hallo` — display a simple banner
- `type` — check whether a command is built-in
- `complete` — register or inspect custom completions
- `exit` — quit the shell
- `jobs`- helps to see all running and done jobs which are in the background
- `history` - see all the command you typed till now from the start of the shell
- `declare` - helps to declare a shell variables


## Output Redirection

So it Support the stdout and stderr Redirection with `>`,`1>` and `2>` respectively and that to be for both the built-ins and the external commands

the logic is present inside the executor.rs nad the redirect.rs so in the exector first its seacrhed is there any redirection if it present then its pased to the redirect where the output of the command is written or appended in the file given if the file not present then its created and stored and this redirect command is handedled differently form the normal command so they are divided by a if-else branch

## Pipelines

Before creating this feature i did'nt knew about how it works so it took me quite a amount of time to understand what is this and how does it works 

so let me explain what does this `|` pipline do when its between two commands basically the stdout from the first command is given as the stdin to the second command and the both command are running at a time its not like first the first command runs then after that the second command runs and it works same for when there is multiple commands and in between them there is pipline

the code is pipline.rs so first in the REPL checked if there is a pipline in the command then its passed to this file then the whole command is splited with `|` then inside this they are stored each command with its argument `cmds_and_args = Vec::<(&str, Vec<&str>)>::new()` now you might wonder why had not used hashmap its because i need the order and here order is necessary so i had used this way then i there is previous_stdout which ther to store the previous command stdout which can be used to pipe the it to the current stdin

So let me explain it for Dual command first both commands are spawned with there args provided then the first command stdout a pipe is created using Stdio::piped() and then in the second command inside its stdin its taken like this command1.stdout.take and is passed to stdin so this way its given as a stding to the next command

Now for muliticommand pipline as i said earlier i stored the preivous stdout and pass it to the current stdin by creating a pipe and taking the stout until its the last child then for the last commad its stdout is given to terminal where the whole output is printed

## Background Jobs

before executing the command its checked after split with wihitespace is its last element is `&` then its classified as a background job and then its runned using .spawn() nad stored in a vec which stores all the background jobs currenlty running and to see which is the latest backround job used the vecs len - 1 to get the latest and -2 for the second latest backgound job as they are showned using + nad - respectively and when the background job is completed its removed from the vec

to see all the backround jobs use `jobs` command basically its just print that vec its also stores the child and the job no in that vec its like a tuple at and here also i had not used hashmap as i need the order for getting the latest and second latest jobs

## Tab Completion

for the builtin and external command completion i just provided all the command start with how much command had typed before tab all possible are given to rustyline completion then it handel the completeion similarly done for file and directories completion checked if thats a directory then its ended with a `/` for if its a file its ended with nothing so for checking if its a directory for file its checked using its metadata if its a file or a directory most of the autocompletion and key binding is handeled by the rustyline is just needed provide it some resorses and it does the job 

## History

Making this feature was prettey straight forward created a vec of strings so before the command goes to match statement in the REPL it its appended in this vec and in rustyline vec also so that it can respond to the up and down arrow nad as all the command typed is stored in the hostory vec so it can be written inside the file or if there is already and there is append command then for this this logic its little curvy forwarded so i can just not add all the command i need to append that much commnads which was written after the previous history append command else if there is no previous history append command in that file than just append all the history command

## Parameter Expansion 

its also implementing was easy as i created a hashmap to store the key and the value which was created using `declare` command and i need to implenent it like if that key is used with `$` or `${}` then i need replace it with the value there for the key so in both builtin and external command i check if there is any arg in the command which like the above pattern starts with dollar then its seached in the hashmap if the key is present then its repalced with the value else the `$` and `${}` is replace by empty `''` its one of the easiest feature in this whole shell project

# So hope you liked this project if read up to here so here is something i faced and learned

## **Challenges & What I had learned in this Whole Journey**
So for Challenges i would say this whole project was a challenge for me at first but the most challenging parts were 
- handling external commands
- echo Quoting
- piplines
- backround jobs and output redirection in external commands

Things i have learned in this its that i came to know so many linux commands and shell commands how pipelines works and how other commands so system programming concepts handling back jobs and much more