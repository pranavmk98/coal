# coal

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/pranavmk98/coal/Rust)

`coal` is an alias container manager for the shell. It provides a system to quickly add/remove shell aliases, isolate aliases into containers, quickly switch between, create, and delete containers.


## Motivation

The inspiration for this idea came from working on multiple projects at once - when juggling various tedious build commands, things can quickly get unwieldy. The natural solution is to introduce aliases in one's `.bashrc` (or dotfile of their choosing). To avoid the mental overhead in coming up with unique aliases across contexts, a containerized solution was the most appealing.

## Installation

Installing `coal` requires [Cargo](https://crates.io/) to be installed. Installation may take up to a minute

```
$ git clone git@github.com:pranavmk98/coal.git
$ cd coal
$ chmod +x setup.sh
$ ./setup.sh
Checking for cargo... complete!
Using source file: /home/<user>/.zshrc
Installing dependencies and binary... complete!
Setting up environment... complete!
Setting up source file... complete!
Setup complete. Restart your shell for the new changes to take effect.
```

Supported shells: `bash`, `zsh`, `ksh`

## Usage

Create a new alias container called `server`:
```
$ coal new server
```

Add a new alias `hw` to run `echo 'Hello, World'`:
```
$ coal add hw "echo 'Hello, World'"
$ hw
Hello, World
```

Create a fresh new container `client` with no aliases (and automatically switch to it):
```
$ coal new client
```

Show existing containers and the currently active one:
```
$ coal show
server
client*
$ hw
zsh: command not found: hw
```

Show existing aliases in the `server` container:
```
$ coal show server
hw -> "echo 'Hello world'"
```

Switch to the `server` container:
```
$ coal load server
$ coal show
server*
client
$ hw
Hello, World
```

Delete the `client` container:
```
$ coal delete client
$ coal show
server*
```

Remove the `hw` alias:
```
$ coal rem hw
$ hw
zsh: command not found: hw
```

## Contributing

This project has been lightly tested on `bash`, `zsh`, and `ksh`. I've tried to
make this tool as robust as possible, but if there's something I missed I'd love
your help.

* Want a new feature? Feel free to file an issue for a feature request.
* Find a bug? Open an issue please, or even better send me a pull request.

Contributions are always welcome at any time!
