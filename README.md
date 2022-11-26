# Quick translator

## Summary
This is a small application made in rust that aims at translating some text in different languages very fast.

It uses the [Lingva](https://lingva.ml) api for the translation.

## Build

You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, you can compile the program by running the following in a terminal:

`cargo build --release`

Then the executable can be found in `target/release`.

## How to execute the program

The idea is to bind this program to a specific shortcut so that you can run it quickly.

### Example for **Linux** users

For example, you can put the executable in the local `bin` folder:

`mv ./target/release/quick-translator ~/.local/bin`

Then make sure that it's in the `PATH` variable by typing:

`echo $PATH`

If it's not, you can add it by adding at the end of `~/.profile` :

`export PATH=$PATH:$HOME/.local/bin`

Then, you can bind a shortcut using your DE or WM's features to the `quick-translator` command.

### For **Windows** users

You can pin the program in your taskbar. In that way, you can run it using Windows's default shortcut. For instance, if it's in the n-th position of your taskbar, you can run it by pressing `WIN + n`.

## TO-DO
For the moment, the program will start in coordinates `1650, 30` no matter what screen resolution you have. The objective for the next version is to start the program at the top-right of the screen depending of your resolution.