# Documentation

## Configuration

### Date formats

Date formatting is done using the [Chrono](https://docs.rs/chrono/latest/chrono/) crate. The available formats can be found here: [strftime specifiers](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).

### Icons

If you are using [NerdFont](https://www.nerdfonts.com/), you can search for icons using [their tool](https://www.nerdfonts.com/cheat-sheet).

### Colors

The available colors are rust's [TUI](https://docs.rs/tui/latest/tui/) styles. The list of colors is the following:

- `Reset`
- `Black`
- `Red`
- `Green`
- `Yellow`
- `Blue`
- `Magenta`
- `Cyan`
- `Gray`
- `DarkGray`
- `LightRed`
- `LightGreen`
- `LightYellow`
- `LightBlue`
- `LightMagenta`
- `LightCyan`
- `White`
- `Rgb(u8, u8, u8)` -> Example: `Rgb(255, 0, 0)`
- `Indexed(u8)` -> Example: `Indexed(3)`

### Key Bindings

The available keys correspond to rust's crate [Crossterm](https://docs.rs/crossterm/latest/crossterm/) keycodes. The list of all KeyCodes can be found here: [KeyCode enum](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html). Not all KeyCodes have been implemented. Maybe I will add more in the future.

The currently available list of KeyCodes is:

- `Esc`: the escape key
- `Backspace`: the backspace key
- `Left`: the left arrow key
- `Right`: the right arrow key
- `Up`: the up arrow key
- `Down`: the down arrow key
- `Home`: the home key
- `End`: the end key
- `Delete`: the delete key
- `Insert`: the insert key
- `PageUp`: the page up key
- `PageDown`: the page down key
- `F1` to `F12`: the function keys
- `Space`: the space key
- `Tab`: the tab key
- `Enter`: the enter key
