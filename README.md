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

## Configuration repository

`configurator` will create two directories in the `CONFIGURATOR_REPO` as necessary:

* **`home`** - contains all configuration files that live under the user's home directory (`~`). This allows `configurator` to work across different computers with different usernames. File paths within `home` are relative to `~`.
* **`root`** - contains all files that don't belong in `home`. File paths within `root` are relative to `/`.

The contents of these directories are also used to determine what files `configurator` manages.

Note that if you are using a git repository as your `CONFIGURATOR_REPO`, you still need to commit your changes after running `add`/`stage`. This is slightly inconvenient because it essentially adds a third stage to the commit process, but I think the extra control and protection against accidental data loss is valuable.

## Usage

All paths and references to `FILE` refer to the system path, not the repository path.

### Adding new files
To add a new file to the repository, use `configurator add FILE`. Using `add` on a file that is already being managed by `configurator` overwrites the repository file with the system file.

### Installing files onto the system
To copy all files from the repository to the system, use `configurator install`. This process is interactive to prevent accidental data loss. To install a specific file, use `configurator install FILE`.

### Updating the repository
To refresh the contents of the repository with any updated system files, run `configurator stage`. I assume you are using a git repo as your repository which gives some safety to repository updates, so this command is not interactive. Use caution.

### Querying the repository
To get a quick list of any modified files, use `configurator status`. It would be cool eventually to use the git index to determine if files were changed on the system or in the repo, but for now all that is possible is showing that differences are present.

To get a diff of all modified files, use `configurator diff`. To get a diff of a single file, use `configurator diff FILE`. It is assumed that the repository is the original and the system files are the modification when deciding `-`/`+` lines in the diff.

To get a list of all system files managed by `configurator`, use `configurator list`. I don't currently have a use case for this, but it might come in handy for some script some day.

### Managing root-owned files
To work with files which your user does not have read/write permission on, include the `--root` flag: `configurator --root add FILE`. You will be prompted for your password.

In my experience, this is only required when `install`ing write protected system files, i.e. anything in `/etc/`.

#### why no sudo
By default, `cargo install` installs the executable into `~/.cargo/bin`, which is on your user's PATH. This means that when trying to run commands as root, for example, `sudo configurator add /etc/hosts`, the `configurator` executable cannot be located.

If you're aware of a better solution, do let me know.

## Testing

Tests must be invoked like this:
```sh
cargo test -- --test-threads 1
```
This ensures that tests are run serially. Several tests reference the same files and require that the file be present/absent, so it's not possible for them to run in parallel.
