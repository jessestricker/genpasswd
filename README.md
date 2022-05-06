# genpasswd

An easy-peasy password generator for the command line.

## Features

- **simple:** The application has intentionally few functions 
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
    -n, --length <LENGTH>    The length of the generated password
    -t, --type <TYPE>        The set of characters to choose from [default: ascii] 
                             [possible values: letters, digits, letters-digits, ascii]
    -V, --version            Print version information
```
