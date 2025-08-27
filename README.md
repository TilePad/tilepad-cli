<table>
<tr>
<td>
<img src="https://github.com/TilePad/tilepad-desktop/raw/main/assets/tilepad-logo.svg" width="150px">
</td>
</tr>
</table>

# Tilepad CLI

> Command line tool for working with [Tilepad](https://github.com/tilepad/tilepad-desktop) plugins, packing plugins, linking, ...etc

```
CLI for developing tilepad plugins

Usage: tilepad.exe [OPTIONS] [COMMAND]

Commands:
  create            Scaffold out a new tilepad plugin
  restart           Restart a specific plugin
  stop              Stop a specific plugin
  link              Link the current plugin to tilepad
  unlink            Remove the link from the current plugin
  reload-plugins    Tell TilePad to reload the currently loaded plugins and load any new plugins that were added
  bundle            Bundles the .tilepadPlugin directory into a .tilepadPlugin archive ready to be installed by Tilepad
  bundle-icon-pack
  help              Print this message or the help of the given subcommand(s)

Options:
  -p, --port <PORT>  Override the default server port when using a custom port within the tilepad desktop app
  -h, --help         Print help
  -V, --version      Print version
```

## Installation

### Install prebuilt binaries via shell script

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tilepad/tilepad-cli/releases/latest/download/tilepad-cli-installer.sh | sh
```

### Install prebuilt binaries via powershell script

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/tilepad/tilepad-cli/releases/latest/download/tilepad-cli-installer.ps1 | iex"
```

### Install prebuilt binaries into your npm project

```sh
npm install @tilepad/cli
```

## Useful commands

Pulling down the git submodules for the various project templates:

```
git submodule update --init --recursive
```

Adding a new template:

```
git submodule add https://github.com/TilePad/tilepad-example-js.git templates/template-javascript
```
