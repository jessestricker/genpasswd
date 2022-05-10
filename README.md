# genpasswd

An easy-peasy password generator for the command line.

## Features

- **simple:** The application has intentionally few features
  and uses sensible defaults.
- **secure:** A small code base with few dependencies eases a security audit.

## Example

```console
$ genpasswd -l 16
Tt/*sD0Y_Sgt6OlK

$ genpasswd -l 24 -t letters-digits -v
info: Characters: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789
info: Length: 24
info: Count: 1
L2AwPVt7XNBclET4BGoMflJC

$ genpasswd -l 6 -t digits -n 4
836634
985309
515779
308633
```

## Usage

```
genpasswd 0.4.0
Jesse Stricker
An easy-peasy password generator for the command line.

The requested number of randomly generated password strings are printed
line-by-line to standard output.

By default, `genpasswd` does **not** enforce that at least one character of each
category (e.g. letter/digit/symbol) picked.

USAGE:
    genpasswd [OPTIONS] --length <LENGTH>

OPTIONS:
    -h, --help
            Print help information

    -l, --length <LENGTH>
            The length of each generated password

    -n, --count <COUNT>
            The number of generated passwords

            [default: 1]

    -t, --type <TYPE>
            The set of characters to choose from

            [default: ascii]
            [possible values: letters, digits, letters-digits, ascii]

    -v, --verbose
            Print information about the generated passwords to standard error

    -V, --version
            Print version information
```
