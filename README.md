# discwhois
discwhois is a console application written in Rust. It is used to find some public information on a user given an ID.

## Purpose
Why use this CLI if I already have websites that do it? Using this program is much faster than going to a website to do it for you, simply open a console and use the program.

Not only that, but this was more of a learning project, since I am aspiring to be a programmer.

## Usage
To use the program, add it to your path or run it like any normal Command Line Application.
```
$discwhois <id>
```

It has both `--help` and `--version` flags.
```
$discwhois --help
discwhois 0.1.0

USAGE:
    discwhois <id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <id>
```

```
$discwhois --version
discwhois 0.1.0
```

## Functionality
The program makes an API call to the Discord API to retrieve the main info (username, discriminator, avatar URL). The creation date is obtained through the ID itself, since Discord uses snowflakes which contain their creation date in epoch minus the discord epoch.

## Building
```
$git clone https://github.com/TixoLan/discwhois.git
$cd discwhois
$cargo build --release
```
After this, add `.../discwhois/target/release/discwhois.exe` to your `PATH` so you can use it on any directory.
