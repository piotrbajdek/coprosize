# INTRODUCING COPROSIZE

[coprosize](https://github.com/piotrbajdek/coprosize) uses power, exponential, and cubic regression models to estimate the body mass of the producer based on coprolite diameter. These models can be selected based on the suspected taxonomy of the producer (currently limited to tetrapod models) and its dietary type (carnivorous, herbivorous, omnivorous, or unspecified). The regression formulae utilised are described in detail in [Supplement 1. Regression models](https://github.com/piotrbajdek/coprosize/blob/main/docs/supplement-1.ods) and were constructed using data found in [Supplement 2. Scat diameters and body masses](https://github.com/piotrbajdek/coprosize/blob/main/docs/supplement-2.ods).

To meet the demands of the scientific community, coprosize was created using Rust. This choice was made for the following reasons: to ensure the code produced has a high level of accuracy, to maintain the accessibility of program versions in the [registry](https://docs.rs/crate/coprosize/latest) forever, and to make it easy to install and use on various platforms due to Rust's strict policy of maintaining backwards compatibility.

[keywords (en-AU): animal scats, archaeology, biology, body mass, body size, coprolites, dung, fossil faeces, fossils, geology, ichnology, palaeontology, scientific computing, weight; (en-US): fossil feces, paleontology]

# USAGE

![help-image](https://github.com/piotrbajdek/coprosize/blob/main/docs/images/help-image.png?raw=true)

# EXAMPLES

![example-image-1](https://github.com/piotrbajdek/coprosize/blob/main/docs/images/example-image-1.png?raw=true)

![example-image-2](https://github.com/piotrbajdek/coprosize/blob/main/docs/images/example-image-2.png?raw=true)

# CITATION AND REUSE

When citing coprosize in research, it's essential to mention the exact program version. The formulae implemented may change with the availability of new data or bug fixes. coprosize is designed for ease of use, but it's still appropriate to use the models in your research without installation, as long as proper credit is given to the software as the source.

Bajdek, P., 2023. coprosize (version 1.0.4). [computer software] https://github.com/piotrbajdek/coprosize

coprosize can be modified and forked under the terms of the [MIT License](https://github.com/piotrbajdek/coprosize/blob/main/LICENSE.md). It is moreover possible to link against coprosize using its library as a dependency for other bioinformatics projects (see [public functions](https://docs.rs/coprosize/1.0.4/coprosize/#functions)). The usage is best explained by example:

Add to your `Cargo.toml` file:

```
[dependencies]
coprosize = "1.0.4"
```

Put in your `src/main.rs` file:

```
fn main() {
let diameter = "9"; // diameter as &str
coprosize::herbivorous_rodentia(diameter);
println!();
coprosize::omnivorous_rodentia(diameter);
println!();
println!("Source of the above models:");
println!();
coprosize::citation();
}
```
This program will utilise coprosize's internal library. It's important to note that the 'diameter' must be provided as a _string slice_ (of numeric characters and a dot, if necessary). The library won't function if either an _integer_ or a _floating point_ value is given.

# INSTALLATION ON LINUX

[coprosize](https://github.com/piotrbajdek/coprosize) is designed to be compatible with **Windows** and **macOS**, and can be easily installed using [cargo](https://www.rust-lang.org/tools/install). However, the primary development and testing environment for coprosize is **Fedora Linux**.

The current version of coprosize (v1.0.4) has been verified to work properly on Fedora Linux 37 and Ubuntu 22.10.

## METHOD 1 – USING CARGO

**[Recommended for programmers]**

**1.** To install coprosize from [crates.io](https://crates.io/crates/coprosize), use the following cargo command:

_cargo install coprosize_

The executable will be saved in the hidden `.cargo/bin/` directory within your home directory.

**2a.** For easy access, you may want to copy the coprosize file to the `/usr/bin/` directory. This can be done by following the instructions in Method 2 (3a, 3b).

**2b.** As an alternative, you can add the `~/.cargo/bin/` directory to your system's PATH variable, which can be configured using [rustup](https://www.rust-lang.org/tools/install).

## METHOD 2 – UNIVERSAL LINUX BINARIES

**1.** To install coprosize, first download the distro-independent [binary](https://github.com/piotrbajdek/coprosize/releases/download/v1.0.4/coprosize) from GitHub.

**2.** Then, make the file executable by running the command:

_sudo chmod +x ./coprosize_

**3a.** On most Linux distributions, install coprosize by copying the binary to `/usr/bin/`:

_sudo cp coprosize /usr/bin/_

**3b.** For Fedora Silverblue / Kinoite, use this command:

_sudo cp coprosize /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[Recommended for most users]**

Distro-specific packages for [.rpm](https://github.com/piotrbajdek/coprosize/releases/download/v1.0.4/coprosize-1.0.4-1.x86_64.rpm) and [.deb](https://github.com/piotrbajdek/coprosize/releases/download/v1.0.4/coprosize_1.0.4_amd64.deb)-based Linux distributions are also available for download. To install coprosize on different Linux distributions, follow these instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i coprosize-1.0.4-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install coprosize-1.0.4-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i coprosize_1.0.4_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

First, download and unpack the coprosize [source code](https://github.com/piotrbajdek/coprosize/archive/refs/tags/v1.0.4.zip) from GitHub. Next, to build and install the program, use the command:

_cargo build \--release && sudo cp target/release/coprosize /usr/bin/_
