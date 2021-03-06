# Description

*Sonny* is a functional programming language designed to create music and other sounds. It features a highly modular, fully programmable sound generation and transformation pipeline so that the user has complete control over what is generated.

#### Important Note

**Sonny is not currently undergoing any new development. I have found that there is a fundamental problem with using an interpretted language to process audio: it is far too slow. You may be interested in a more recent music programming project of mine: [Ryvm](https://github.com/kaikalii/ryvm).**

# Features

### Current Features

* Standard programming language expression evaluation including arithmetic, logical, and ternary operators
* Modular arithmetic sound transformation via function-like constructs called "chains"
* Easy-to-type note entry to build song loops
* Song loop arrangement via chains
* Frequency-domain sound manipulation (not perfect)
* Simple but effective module system for separating code into multiple files or libraries
* Compiles to .WAV format

### Originally Planned Features (likely never going to happen)

* Output to other audio formats, namely .MP3 and .OGG
* Playback and manipulation of external audio samples
* A more powerful module system for more complex libraries
* Continuous sound playback from the compiler itself
* Proper language documentation (currently there is only a lackluster grammar file)
* Getting user input to allow for interactive programs
* A decent standard library with lots of useful chains

# Design Goals

### Goals

The language should (eventually)...

* be capable of creating any audio file that a typical DAW (Digital Audio Workstation) program like Ableton, Reason, or Logic are capable of making.
* be easy to use for the purposes of sound design and audio generation.
* be fast. A decent machine should be capable of continuous audio generation and playback without any pauses.
* have simple, nice-looking syntax. (This is obviously subjective.)
* be compatible with all major desktop operating systems.
* not require a GUI for the vast majority of features.

### Non-Goals

The does not need to, and probably should not...

* be easy to use as a general-purpose programming language. There are plenty of very good general-purpose programming languages already. *Sonny* only seeks to be good in its niche of sound processing.

# Installation

#### Requirements

* Git (recommended)
* Rust
* Cargo

The *Sonny* compiler is written in the Rust programming language. Currently, the main way to install *Sonny* is to build it from source. Luckily, Rust makes this very easy. Before you can install *Sonny* you will need to install Rust and its package manager, cargo.

If you do not already have *Rust* installed, head over to [the *Rust* website](https://www.rust-lang.org/) and install it. Once cargo is installed, simply run these commands in your terminal:
```
git clone https://github.com/kaikalii/sonny.git
cd sonny
cargo build --release
```
Then, you can run the example:
```
cargo run example.son --play
```
After this, you can `cargo build --release` and add /PATH/TO/sonny/target/release to your path if you want to be able to compile *Sonny* projects from any directory.

If you use Atom as your editor, you can get syntax highlighting for *Sonny* by installing the [language-sonny](https://github.com/kaikalii/language-sonny) package.

# Documentation

To learn the basics or programming in *Sonny*, head over to [the documentation](https://kaikalii.github.io/sonny/).

# Contributing

Any contribution to *Sonny*'s development is greatly appreciated. If you would like to contribute, please keep the following in mind:

* Unless your contribution is a simple bug fix, open an issue to discuss it before submitting a pull-request.
* Please use rustfmt on your code with the default settings.
