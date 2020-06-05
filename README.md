# lenv

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/pranavmk98/alienv/Rust)

`lenv` is an alias environment manager for the shell. It provides a system to
containerize shell aliases into isolated environments, quickly switch between environments,
and create/delete environments.


## Installation

Installing `lenv` requires [Cargo](https://crates.io/) to be installed.

```
$ git clone git@github.com:pranavmk98/alienv.git
$ cd alienv
$ chmod +x setup.sh
$ ./setup.sh
Setup complete. Restart your shell for the new changes to take effect.
```

## Usage

Create a new alias environment called `server`:
```
$ lenv new server
```

Add a new alias `hw` to run `echo 'Hello, World!'`:
```
$ lenv add hw "echo 'Hello, World!'"
$ hw
Hello, World!
```

Create a fresh new environment `client` with no aliases (and automatically switch to it):
```
$ lenv new client
```

Show existing environments and the currently active one:
```
$ lenv show
server
client*
$ hw
zsh: command not found: hw
```

Switch to the `server` environment:
```
$ lenv load server
$ lenv show
server*
client
$ hw
Hello, World!
```

Delete the `client` environment:
```
$ lenv delete client
$ lenv show
server*
```

Remove the `hw` alias:
```
$ lenv rem hw
$ hw
zsh: command not found: hw
```

## Motivation

The inspiration for this idea came from working on multiple projects at once - when juggling various tedious build commands, things can quickly get unwieldy. The natural solution is to introduce aliases in one's `.zshrc` (or dotfile of their choosing). However, that would require ensuring that these aliases are all distinct. Perhaps a numbering system like `build1`, `build2` would suffice, but a containerized solution is very appealing and would reduce this mental overhead involved in coming up with unique alias names across different contexts.

## Issues

Please submit issues [here](https://github.com/pranavmk98/alienv/issues), and always feel free to create PRs!