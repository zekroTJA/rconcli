# rconcli

A little CLI to issue RCON commands to a Minecraft server.

![WindowsTerminal_ClQRborIho](https://github.com/zekroTJA/rconcli/assets/16734205/7a0ee0e5-0627-4f35-bcc6-1c34dbd07722)

## Installation

You can either download the latest pre-compiled binaries from the [releases page](https://github.com/zekroTJA/rconcli/releases).
You can also install the tool via cargo.

```
cargo install rconcli
```

## Usage

```
$ rconcli --help
A simple RCON CLI for Minecraft servers.

Usage: rconcli.exe [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  Command to execute

Options:
      --properties <PROPERTIES>  Location of a server.proterties file to read credentials from
  -a, --address <ADDRESS>        The address and port of the target server
  -p, --password <PASSWORD>      The password of the target server
      --no-color                 Supress colored output
  -h, --help                     Print help
  -V, --version                  Print version
```