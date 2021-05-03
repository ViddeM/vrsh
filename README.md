# vrsh
A small shell written in rust as a way for me to learn rust

Long term (most likely dream-like) goal, achieve POSIX compliance (https://pubs.opengroup.org/onlinepubs/009604499/utilities/xcu_chap02.html).

# Current & planned features
Currently only a small amount of features have been implemented but more are planned, feel free to suggest features to be added to the planned features list!

## Current & planned features list:
 - [x] Execution of programs with given arguments.
 - [ ] Support single quotes `'`
 - [ ] Arguments without spaces should be one argument e.g. `"asd""bsd"` should be one argument.
 - [ ] Support environment variables e.g. `$HOME`
 - [x] Support comments `#`
 - [x] Command history:
   - [x] Persistant (stored in history file in users home directory).
   - [x] Can reuse commands by using `arrow-up` to go back in history and `arrow-down` to go forward in history.
 - [ ] Some basic highlighting:
   - [x] Suggest previously used commands.
   - [ ] Highlight (valid) commands.
   - [ ] Highlight strings.
   - [ ] Matching expansions
 - [ ] Expansions (Look through https://www.gnu.org/software/bash/manual/html_node/Shell-Expansions.html for more):
   - [x] Command expansions using `$()`.
   - [ ] * -> any file matching. 
   - [ ] More... see above url
 - [x] Piping between programs `|`.
 - [x] Redirects:
   - [x] From command output to file (`>`).
   - [x] To program from file (`<`).
 - [ ] Built in commands:
   - [x] `cd`
   - [x] `exit`
   - [x] `alias`
      - [x] `~` -> the home directory of the current user.
   - [ ] `source`
   - [ ] variables i.e. `A="some value"`
      - [ ] Program specific variables i.e. `SOME_VARIABLE="some_value" firefox`
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
   - [ ] Support starship integration: https://starship.rs/.
 - [ ] Configuration using a file.
   - [ ] Be able to add aliases.
   - [ ] Configure color scheme.
   - [ ] Customize prompt
 - [ ] Background processes `&`
 - [ ] Sequentially executed commands `&&`
