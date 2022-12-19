# configurator

A simple CLI tool for copying system files into and out of a central location.

I have multiple computers running Linux that all have their own array of configuration and system files (`.bashrc`, `.vimrc`, `i3`, font files, etc.). Manually updating and propogating changes between systems to maintain a consistent experience is quite painful. `configurator` solves this problem by synchronizing your files between your system and a central location (I use a [git repository](https://github.com/dchiquito/configurations)).

## Installation

You will need `rust`/`cargo` installed to install `configurator`.

```sh
cargo install --git https://github.com/dchiquito/configurator.git
```

To install from source:
```sh
cargo install --path .
```

You will also need a directory to keep your configuration files. I recommend a git repository or an NFS mount.

Add the following to your `.bashrc`/`.zshrc`/`config.fish`:
```sh
export CONFIGURATOR_REPO=/my/configuration/directory/
```

You can also specify the repository when running `configurator`:
```sh
configurator --repo /my/configuration/directory/ ...
or
configurator -r /my/configuration/directory/ ...
```

### Warning: `sudo`

`cargo install` installs the crate into `~/.cargo/bin`, which is not on the `PATH` when running as `sudo` (unless you installed/ran cargo as `root`). Sadly, if you have any root-owned configuration files, then you will need to run `configurator` as root. In lieu of a proper solution, I just manually run `sudo cp ~/.cargo/bin/configurator /usr/bin/`. YMMV.

## Configuration repository

`configurator` will create two directories in the `CONFIGURATOR_REPO` as necessary:

* **`home`** - contains all configuration files that live under the user's home directory (`~`). This allows `configurator` to work across different computers with different usernames. File paths within `home` are relative to `~`.
* **`root`** - contains all files that don't belong in `home`. File paths within `root` are relative to `/`.

## Commands

All paths and references to `FILE` refer to the system path, not the repository path.

* `configurator list` - List all managed files in the repository.
* `configurator add FILE` - Copy `FILE` from the system into the repository.
* `configurator stage` - Update every file in the repository with its system equivalent.
* `configurator install [FILE]` - Copy FILE from the repository onto the system. If no `FILE` is specified, copy all files.

Note that if you are using a git repository as your `CONFIGURATOR_REPO`, you still need to commit your changes after running `add`/`stage`.
