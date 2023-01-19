# INTRODUCING COPROSIZE

[coprosize](https://github.com/piotrbajdek/coprosize) employs power, exponential and cubic regression models allowing to estimate the producer's body mass based on coprolite diameter. Models can be chosen accordingly to the supposed producer's taxon (at this stage of program development, only tetrapod models are implemented) and its diet type (carnivorous, herbivorous, omnivorous, unspecified). Implemented regression formulae are provided in [Supplement 1. Regression models](https://github.com/piotrbajdek/coprosize/blob/main/docs/supplement-1.ods) and constructed based on the data given in [Supplement 2. Scat diameters and body masses](https://github.com/piotrbajdek/coprosize/blob/main/docs/supplement-2.ods).

As it is aimed for science, [coprosize](https://github.com/piotrbajdek/coprosize) is written in Rust. The reasons for this choice are (1) the high code correctness guaranteed by Rust, (2) to ensure that each program version will be accessible in the [registry](https://docs.rs/crate/coprosize/latest) 'in perpetuity' and (3) that each program version will remain easily installed and cross-platform 'in perpetuity', thanks to the Rust's strict policy of backwards compatibility.

[keywords (en-AU): animal scats, archaeology, biology, body mass, body size, coprolites, dung, fossil faeces, fossils, geology, ichnology, palaeontology, scientific computing, weight; (en-US): fossil feces, paleontology]

# USAGE

![help-image](https://github.com/piotrbajdek/coprosize/blob/main/docs/images/help-image.png?raw=true)

# EXAMPLES

![example-image-1](https://github.com/piotrbajdek/coprosize/blob/main/docs/images/example-image-1.png?raw=true)

![example-image-2](https://github.com/piotrbajdek/coprosize/blob/main/docs/images/example-image-2.png?raw=true)

# CITATION AND REUSE

Please, always refer to a specific program version--implemented formulae are subject to change if new data are available (or simply studied by the author) or bugs of any kind are detected. Although coprosize is designed with the needs of a user in mind, you are perfectly OK to use my models in your study without really installing it as long as you cite this computer program as the original source:

Bajdek, P., 2023. coprosize (version 1.0.3). [computer software] https://github.com/piotrbajdek/coprosize

You are also OK to modify and fork coprosize under terms of the [MIT License](https://github.com/piotrbajdek/coprosize/blob/main/LICENSE.md). It is moreover possible to link against coprosize using its library as a dependency for other bioinformatics projects (see [public functions](https://docs.rs/coprosize/1.0.3/coprosize/#functions)). The usage is best explained by example:

Add to your `Cargo.toml` file:

```
[dependencies]
coprosize = "1.0.3"
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
The above program will reuse the internal library of coprosize. Note that 'diameter' must be given as a _string slice_ (of numbers and optionally including a dot)--the library won't work receiving neither an _integer_ nor a _floating point_.

# INSTALLATION ON LINUX

[coprosize](https://github.com/piotrbajdek/coprosize) should run smoothly on **Windows** and **macOS**, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and primarily tested on **Fedora Linux**.

coprosize v1.0.3:

– Was successfully tested on Fedora Linux 37, openSUSE Tumbleweed, and Ubuntu 22.10.

– Failed to run on Mageia 8 due to an old glibc version (required ≥2.34).

## METHOD 1 – BY THE USE OF CARGO

**[recommended for programmers]**

**1.** Install from crates.io by the use of cargo:

_cargo install coprosize_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy coprosize to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (can be set up by [rustup](https://www.rust-lang.org/tools/install)).

## METHOD 2 – LINUX UNIVERSAL BINARIES

**1.** Download the distro-independent [binary](https://github.com/piotrbajdek/coprosize/releases/download/v1.0.3/coprosize) of coprosize from GitHub.

**2.** Make the file executable:

_sudo chmod +x ./coprosize_

**3a.** On most Linux distros, install coprosize via copying the binary to `/usr/bin/`:

_sudo cp coprosize /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp coprosize /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[recommended for most users]**

Distro-specific packages are also available for download for [.rpm](https://github.com/piotrbajdek/coprosize/releases/download/v1.0.3/coprosize-1.0.3-1.x86_64.rpm)- and [.deb](https://github.com/piotrbajdek/coprosize/releases/download/v1.0.3/coprosize_1.0.3_amd64.deb)-based Linux distros. Installation instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i coprosize-1.0.3-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install coprosize-1.0.3-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i coprosize_1.0.3_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

Download and unpack the coprosize [source](https://github.com/piotrbajdek/coprosize/archive/refs/tags/v1.0.3.zip) from GitHub. Then, build and install the program:

_cargo build \--release && sudo cp target/release/coprosize /usr/bin/_
