# genpasswd

An easy-peasy password generator for the command line.

## Features

- **simple:** The application has intentionally few features
  and uses sensible defaults.
- **secure:** A small code base with few dependencies eases a security audit.

## Usage

```
genpasswd 0.3.1
Jesse Stricker
An easy-peasy password generator for the command line.

USAGE:
    genpasswd [OPTIONS] --length <LENGTH>

OPTIONS:
    -h, --help               Print help information
    -l, --length <LENGTH>    The length of each generated password
    -n, --count <COUNT>      The number of generated passwords [default: 1]
    -t, --type <TYPE>        The set of characters to choose from [default: ascii] 
                             [possible values: letters, digits, letters-digits, ascii]
    -v, --verbose            Write information about the generated passwords to `stderr`
    -V, --version            Print version information
```
