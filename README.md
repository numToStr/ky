<h1 align="center">
    <code>🔑 ky</code>
</h1>
<p align="center"><b>Simple & Secure password manager</b></p>

<p align="center">
  <a aria-label="build" href="https://github.com/numToStr/ky/actions/workflows/build.yml">
    <img alt="build" src="https://github.com/numToStr/ky/actions/workflows/build.yml/badge.svg">
  </a>
  <!-- <a aria-label="docs" href="https://docs.rs/zenv"> -->
  <!--   <img alt="docs" src="https://docs.rs/zenv/badge.svg"> -->
  <!-- </a> -->
  <a aria-label="crates.io" href="https://crates.io/crates/ky">
    <img alt="crates.io" src="https://img.shields.io/crates/v/ky.svg">
  </a>
</p>

> **EXPERIMENTAL**: This project is very much in development and can introduce breaking changes at any moment. Also, I am not an expert in cryptography, if you know something about cryptography then I will be more than happy to recieve your feedback.

## 🚀 Installation

#### Using `cargo` (Linux/macOS/Windows)

-   See [#5](https://github.com/numToStr/ky/issues/5)

#### Using `yay` or `pamac` (Arch Linux)

```bash
# Using `yay`
yay -S ky

# Using `pamac`
pamac build ky
```

#### Using release binaries (Linux/macOS/Windows)

Check out the [release page](https://github.com/numToStr/ky/releases) for prebuild binaries for `ky`, available for different operating systems.

<!-- NOTE: `ky` uses symlinks underneath to manage aliases. So, If you are using **Windows** make sure you have enabled **Developer Mode** or your user has permission to create symlinks. You can read more [here](https://blogs.windows.com/windowsdeveloper/2016/12/02/symlinks-windows-10/) -->

## 🤞 Usage

#### Global Options/Flags

| Options         | Env Variable    | Description                           |
| --------------- | --------------- | ------------------------------------- |
| `--vault-path`  | `KY_VAULT_DIR`  | Path for the backup/export directory  |
| `--backup-path` | `KY_BACKUP_DIR` | Path for the vault directory          |
| `--git-repo`    | `KY_GIT_REPO`   | Git repo used as a storage for backup |
| `--git-branch`  | `KY_GIT_BRANCH` | Default branch for the git repo       |

| Flags       | Description         |
| ----------- | ------------------- |
| `--version` | Prints the version  |
| `--help`    | Prints the help doc |

#### Basics

-   Initializing the vault (**required**)

Everything is stored in a small [lmdb](https://github.com/LMDB/lmdb) database. Hence, it is required to initialized the database before saving your password. To initialize the database run `ky init`. After running this, you will be prompted for a master password, make sure you enter a strong and memorable password, as this will be required for every other action afterwards.

```bash
ky init

# help
ky init --help
```

-   Adding an entry

So, you want to crate a password, just run `ky add` with a key which will act as a unique id. After that, you will be asked to enter more details like username, website, expiry-date, notes, which are totally optional for now. After recieving all the details, those will be encrypted with master password and saved into the database.

Remember, password will be auto generated and encrypted with the rest of the details.

```bash
ky add <key>

# help
ky add --help
```

-   Showing an entry

Hmmm...You have added an entry. Now you want to see it. You can do that by running `ky show` with the key. If the key is valid, then the data associated with the key will be decrypted and printed to the stdout.

By default, the password will not decrypted and will shown as hidden. You need to pass `--clear` flag to show the password in clear text.

You can also print the password in a qr-code format by adding `--qr-code` flag.

```bash
ky show <key> # password will be hidden
ky show <key> --clear # now password will shown
ky show <key> --qr-code --mute # only print qr-code

# help
ky show --help
```

-   Editing an entry

Oh ho! Made some mistake or want to update the details. Fear not, just run `ky edit` with the key and go through the prompts similar to adding an entry.

By default, password will not touched. But, if you want to regenerate the password, just add `--password` flag then password will be regenerated at the end. Sadly, you can't manually update or edit the password.

```bash
ky edit <key> # just edit the details
ky edit <key> # edit details and also regenerate password

# help
ky edit --help
```

<!-- list           Print a tree view of all keys present in the vault [aliases: ls] -->
<!-- move           Rename the key and re-encrypt the entry's details [aliases: mv] -->
<!-- remove         Remove an entry from the vault [aliases: rm] -->

<!-- backup         Backup the vault -->
<!-- restore        Restore the vault backup -->
<!-- export         Export data as a csv file containing decrypted data -->
<!-- import         Import data from a csv file containing decrypted data -->
<!-- git            Use git to manage the vault -->

<!-- completions    Generate completions for different shells -->
<!-- gen            Generate random and cryptographically strong password -->

<!-- help           Prints this message or the help of the given subcommand(s) -->
<!-- nuke           Permanently delete the local vault -->

## 🔧 Building

`ky` is written in Rust, so you'll need to install the latest Rust toolchain in order to compile it. Visit [rustup.rs](https://rustup.rs/) to download the toolchain.

To build `ky`:

-   Setup

```bash
git clone https://github.com/numToStr/ky

cd ky
```

-   Running

```bash
# same as `ky help`
cargo run -- help

# same as `ky init`
cargo run -- init
```

-   Build

```bash
# debug build
cargo build

# release build
cargo build --release
```
