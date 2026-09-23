# randomstr

A secure random string generator for passwords or tokens, with flags for length, character sets (alphanumeric, special), and no python required.

## Status

**built, untested**: the random string generation is built.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
# Generate a 32-character alphanumeric string
randomstr

# Generate a 64-character string including symbols
randomstr -l 64 -s

# Generate 5 passwords
randomstr -c 5 -s
```
