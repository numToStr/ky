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

### Using `cargo` (Linux/macOS/Windows)

-   See #issue

### Using `yay` or `pamac` (Arch Linux)

```bash
# Using `yay`
yay -S ky

# Using `pamac`
pamac build ky
```

### Using release binaries (Linux/macOS/Windows)

Check out the [Release page](https://github.com/numToStr/ky/releases) for prebuild binaries for `ky`, available for different operating systems.

<!-- NOTE: `ky` uses symlinks underneath to manage aliases. So, If you are using **Windows** make sure you have enabled **Developer Mode** or your user has permission to create symlinks. You can read more [here](https://blogs.windows.com/windowsdeveloper/2016/12/02/symlinks-windows-10/) -->

## 🔧 Building

`ky` is written in Rust, so you'll need to install the latest Rust toolchain in order to compile it. Visit [rustup.rs](https://rustup.rs/) to download the toolchain.

To build `ky`:

```bash
$ git clone https://github.com/numToStr/ky
$ cd ky
$ cargo build --release
$ ./target/release/ky --version
ky 0.0.1
```
