# r-todo
_The world's most complicated todo application_

&nbsp;&nbsp;&nbsp;&nbsp;Accumulates Issues and Todos from multiple sources and syncs keeps them in a local cache.

&nbsp;&nbsp;&nbsp;&nbsp;Currently only outputs through stdout.

## Installation
Compiled and tested using rustc 1.46

Clone the repo and run the following:
```
cargo install --path .
```

## Usage
```
USAGE:
    rtodo [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help             Prints help information
        --no-ui            just dump the cache file to stdout
    -o, --offline          do not refresh non-local issues in the cache
    -r, --refresh-cache    forces a refresh of the non-local issues in the cache
    -V, --version          Prints version information

OPTIONS:
    -f, --cache-file <FILE>    The path of the cache-file to read and write from
    -c, --config <FILE>        The path to a custom config file

SUBCOMMANDS:
    close    close a todo with the given uuid. NOTE: Closes on remote issue do not sync.
    help     Prints this message or the help of the given subcommand(s)
    new      creates a new local issue
```
