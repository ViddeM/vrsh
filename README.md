# vrsh
A small shell written in rust as a way for me to learn rust

# Current & planned features
Currently only a small amount of features have been implemented but more are planned, feel free to suggest features to be added to the planned features list!

## Current & planned features list:
 - [x] Execution of programs with given arguments.
 - [x] Command history:
   - [x] Persistant (stored in history file in users home directory).
   - [x] Can reuse commands by using `arrow-up` to go back in history and `arrow-down` to go forward in history.
 - [x] Some basic highlighting:
   - [x] Suggest previously used commands.
   - [ ] Highlight (valid) commands.
   - [ ] Highlight strings.
   - [ ] Matching calculations
 - [x] Expansions:
   - [x] Command expansions using `$()`
   - [ ] Look through 
 - [x] Aliases
   - [x] `~` -> the home directory of the current user.
 - [x] Piping between programs `|`.
 - [x] Redirects:
   - [x] From command output to file (`>`).
   - [x] To program from file (`<`).
 - [x] Built in commands:
   - [x] `cd`
   - [x] `exit`
 - [ ] Autocompletion
   - [x] For history, see above.
   - [x] File completion using `tab`.
   - [ ] Expanded file completion by tabbing through them when several options are available.
   - [ ] Tab completion for commands in history.
 - [ ] Prompt
   - [x] Shows current path.
   - [ ] Git integration.
   - [ ] Show current user.
   - [ ] Show result of last command.
   - [ ] Prompt on right side as well.
 - [ ] Configuration using a file.
   - [ ] Be able to add aliases.
   - [ ] Configure color scheme.
   - [ ] Customize prompt
