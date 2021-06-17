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

Everything is stored in a small [lmdb](https://github.com/LMDB/lmdb) database. Hence, it is required to initialized the vault before saving your password. To initialize the vault run `ky init`. After running this, you will be prompted for a master password, make sure you enter a strong and memorable password, as this will be required for every other action afterwards.

```bash
ky init

# help
ky init --help
```

-   Adding an entry

So, you want to create/add a password, just run `ky add` with a key which will act as a unique id. After that, you will be asked to enter more details like username, website, expiry-date, notes, which are totally optional for now. After recieving all the details, those will be encrypted with master password and saved into the vault.

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

-   List all the entry

Just run `ky list` to list all the entries saved in the vault. This will only show you the keys without touching the details associated with the respective key.

```bash
ky list

# help
ky list --help
```

-   Renaming the key

For some reason you want to change the key or you made some type and you want to fix that. Run `ky move` provided with old key and new key. Your data will be decrypted using old key and re-encrypted using the new key combined with the master password, and saved in the vault.

```bash
ky move <old_key> <new_key>

# help
ky move --help
```

-   Deleting an entry

If you no longer needed a saved password, then run `ky remove` provided with the key to remove the entry from the vault. This action is permanent.

```bash
ky remove <key>

# help
ky remove --help
```

#### Backup/Restore

-   Backup the vault

Sometimes you want to backup the whole vault, maybe to setup the vault on a different machine. You can use `ky backup` optionally providing a path, which will take the whole vault and other required files, compresses as it is and make a backup file to restore at later point in time.

Keep in mind, that the vault data are totally untouched i.e hashed and encrypted. So, all the data like master password and other passwords, will be the same when you restore the backup vault.

```bash
ky backup # backup vault in the default backup path
ky backup -p <path> # backup in provided path

# help
ky backup --help
```

-   Restore the vault

Backup is done, now you need to restore that backup. Run `ky restore` to restore the vault backup, you can also provide a custom path where the backup is stored. Everything will be restored i.e hashed and encrypted, as it was before the backup like nothing is changed.

```bash
ky restore # restore from default path
ky restore -p <path> # restore from the provided path

# help
ky restore --help
```

-   Export the vault

Making a decrypted export of your vault is also possible and very easy. Just run `ky export` optionally providing a export path, and now you will have `csv` file with all the data **decrypted** inside it.

Beware that all the data is in clear text and anyone with the access to the export file will be able to read your passwords.

```bash
ky export # export in the default path
ky export -p <path> # export in provided path

# help
ky export --help
```

-   Import the vault

You can import your exported data or also any `csv` file which is in a certain format. While import, you'll be asked for a master password and all the data will be **encrypted** using the master password and after encrypting, everything will be saved in the vault.

```bash
ky import # import from the default path
ky import -p <path> # import from the provided path

# help
ky import --help
```

<!-- git            Use git to manage the vault -->

<!-- gen            Generate random and cryptographically strong password -->
<!-- nuke           Permanently delete the local vault -->
<!-- completions    Generate completions for different shells -->
<!-- help           Prints this message or the help of the given subcommand(s) -->

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
