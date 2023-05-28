# vrsh

A small shell written in rust as a way for me to learn rust

Long term goal/dream (not gonna happen), achieve POSIX compliance (https://pubs.opengroup.org/onlinepubs/009604499/utilities/xcu_chap02.html).

# Customizable prompt

To customize the prompt, one can set the `PROMPT` variable.
This can be done using the `set var=val` syntax where `val` is a string, please note that if single quotes (`'`) are used for the string the content of the string is preserved and prompt expansions will be performed on the prompt, see [prompt expansions](#Prompt-expansion).
If double quotes (`"`) are used for the string, the standard expansions will be performed on the string (note that these are evaluated immediately whereas the single quote syntax is evaluated on each prompt display).

To make the prompt configuration permanent one can set the `PROMPT` variable in the `~/.vrshrc` file (generated on application launch if it does not exist).

## Prompt expansion

The following prompt expansions are currently supported:

-   `%%` for the `%` char.
-   `%n` username of the current user.
-   `%F{var}` to set the foreground color of the coming text, var must either be one of the supported colors (see [prompt expansion colors](#prompt-expansion-colors)); 
    or a number in the range 0-255, see [ansi 8-bit coloring](https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit).
-   `%f` resets the foreground color.
-   `%K{var}` Works the same as `%F` but for background color.
-   `%k` resets the background-color.
-   `%g` prints git information according to the built-in git module (used in the [example](#example) below).
-   `%d` or `%/` print the current working directory in full.
-   `%~` prints the current working directory but replaces `/home/current_user` to `~/`.
-   `%~>` prints only the current directory with a leading `/` (e.g. if the current path is `/var/log`, this will print `/log`). If the current working directory is `/home/current_user` it will instead print `~/`.
-   `%-<` prints only the parent directories of the current directory (e.g. if in `/var/log` this will print `/var`). If the current directory is root (`/`) nothing will be printed.

### Prompt expansion colors
To test colors in the current terminal, one can use the `vrsh-colors` 
command which will print all 256 supported foreground and background colors.
The named colors and their numeric value are as follows:
- `red` = 1
- `green` = 2
- `orange` = 3
- `blue` = 4
- `purple` = 5
- `bluegreen` = 6
- `white` = 7
- `gray` = 8
- `darkred` = 9
- `brightgreen` = 34
- `brightblue` = 123
- `pink` = 200
- `yellow` = 220

## Examples
An example of the prompt that I myself currently use is the one below, it is somewhat verbose though.

```set PROMPT='%F{blue} %~%f %F{orange} as %f %F{purple}%n%f %g %F{brightgreen} ❯ %f'```

Gives the following prompt:
![prompt example](https://user-images.githubusercontent.com/16452604/120086134-bf04cb80-c0dd-11eb-8ab7-a7e127b9c8c4.png)

One can utilize the `%~<` and `%~>` options to print the full path but only highlight the current directory, for example:

```set PROMPT='%F{31} %~<%f %F{51}%~>%f %F{214} as %f %F{170}%n%f %g %F{46} ❯ %f'```

Gives the following prompt:

Example output from the `vrsh-colors` command:
![image](https://user-images.githubusercontent.com/16452604/120086187-0db26580-c0de-11eb-8890-c0bc61950560.png)

All screenshots are taken using:
- Terminal: `Konsole`
- Theme: `breath2` 
- Font: `FiraMono Nerd Font 10pt`)

# Current & planned features

Currently only a small amount of features have been implemented but more are planned, feel free to suggest features to be added to the planned features list!

## Current & planned features list:

-   [x] Execution of programs with given arguments.
-   [x] Support single quotes `'`
-   [ ] Arguments without spaces should be one argument e.g. `"asd""bsd"` should be one argument.
-   [ ] Support environment variables e.g. `$HOME`
-   [x] Support comments `#`
-   [x] Command history:
    -   [x] Persistant (stored in history file in users home directory).
    -   [x] Can reuse commands by using `arrow-up` to go back in history and `arrow-down` to go forward in history.
-   [ ] Some basic highlighting:
    -   [x] Suggest previously used commands.
    -   [ ] Highlight (valid) commands.
    -   [ ] Highlight strings.
    -   [ ] Matching expansions
-   [ ] Expansions (Look through https://www.gnu.org/software/bash/manual/html_node/Shell-Expansions.html for more):
    -   [x] Command expansions using `$()`.
    -   [ ] -   -> any file matching.
    -   [ ] More... see above url
-   [x] Piping between programs `|`.
-   [x] Redirects:
    -   [x] From command output to file (`>`).
    -   [x] To program from file (`<`).
-   [ ] Built in commands:
    -   [x] `cd`
    -   [x] `exit`
    -   [x] `alias`
        -   [x] `~` -> the home directory of the current user.
    -   [ ] `source`
    -   [x] variables i.e. `A="some value"`
        -   [ ] Program specific variables i.e. `SOME_VARIABLE="some_value" firefox`
-   [ ] Autocompletion
    -   [x] For history, see above.
    -   [x] File completion using `tab`.
    -   [ ] Expanded file completion by tabbing through them when several options are available.
    -   [ ] Tab completion for commands in history.
-   [ ] Prompt
    -   [x] Shows current path.
    -   [x] Git integration.
    -   [x] Show current user.
    -   [ ] Show result of last command.
    -   [ ] Prompt on right side as well.
    -   [ ] Support starship integration: https://starship.rs/.
-   [x] Configuration using a file.
    -   [x] Be able to add aliases.
    -   [ ] Configure color scheme.
    -   [x] Customize prompt
-   [ ] Background processes `&`
-   [ ] Sequentially executed commands `&&`
