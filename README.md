[![Build Status](https://travis-ci.org/timmyjose/hangman.svg?branch=master)](https://travis-ci.org/timmyjose/hangman)

A simple hangman game in Rust.

The game will try and detect if the OS is a Unix-like OS and try to see if `/usr/share/dict/words` can be used. If not, the bundled file, `words.txt` will be used instead.
The game will automatically load the entire file in memory, and randomly pick the words fulfilling the minimum and maximum word length constraints.

## Usage
```
$ ./hangman [max-word-length = 24] [number-of-attempts = 10] (on UNIX-like systems)
```
or

```
$ hangman.exe [max-word-length = 24] [number-of-attempts = 10] (on Windows)
```

The minimum word length is set at 5 (not changeable). The minimum number of attempts is set at 10 (can be changed), and the maximum at 100 (not changeable).


## Build instructions

Download the zip file or clone the project. Then from the command-line, change into the project's root directory.


```
$ cd hangman
$ cargo build
$ cargo run [max-word-length = 24] [number-of-attempts = 10]
```

Alternatively, the executable located in target/debug/ can be used in the manner shown in the `Usage` section.


