// Copyright 20236 propenster.
// Licensed under the MIT license (http://opensource.org/licenses/MIT)
// This file may not be copied, modified, or distributed
// except according to those terms.
//! # sluggify, Simple slug or clean url generator for Rust.
//! This library is a simple slug generator or url cleaner for Rust.
//! With default settings, you will get an hyphenized, lowercase, alphanumeric version of any string you please, with any diacritics removed, whitespace and dashes collapsed, and whitespace trimmed.
//!
//! For **getting started** with using `sluggify`, see [the `Getting started` section below](#getting-started).
//! For navigating the documentation of the available modules, see [the `Modules` section below](#modules).
//! If you want to contribute to `sluggify`, see [the `Contribute` section in the repo](https://github.com/propenster/sluggify-rs#contribute).
//!
//! # Getting started
//!
//! We explain how to use sluggify step-by-step.
//! Users who already have experience with Rust can skip right to [Step 3: Use sluggify in your project](https://docs.rs/sluggify/#step-3-use-sluggify-in-your-project).
//! Users who already know `sluggify` might want to jump right into the [modules docs](https://docs.rs/sluggify/#modules)
//!
//! ## Step 1: Setting up Rust
//!
//! Rust can be installed following the instruction for [rustup](https://rustup.rs/).
//!
//!
//! ## Step 2: Setting up a new Rust project
//!
//! Since sluggify is a library, you need to setup your own new Rust project to use sluggify.
//! With Rust, projects and their dependencies are managed with the builtin package manager [Cargo](https://doc.rust-lang.org/cargo/index.html).
//! To create a new Rust project, issue
//!
//! ```bash
//! cargo new hello_world --bin
//! cd hello_world
//! ```
//! in your terminal. The flag `--bin` tells Cargo to create an executable project instead of a library.
//! In [this section](https://doc.rust-lang.org/nightly/book/hello-cargo.html#a-new-project) of the Rust docs, you find details about what Cargo just created for you.
//!
//! Your new project can be compiled with
//! ```bash
//! cargo build
//! ```
//! If dependencies in your project are out of date, update with
//! ```bash
//! cargo update
//! ```
//! Execute the compiled code with
//! ```bash
//! cargo run
//! ```
//! If you are new to Rust, we suggest to proceed with [learning Rust](https://www.rust-lang.org/learn) via the Rust docs.
//!
//! ## Step 3: Use sluggify in your project
//!
//! To use sluggify in your Rust project, add the following to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! sluggify = "0.1.0"
//! ```
//!
//! and import the crate from your source code:
//!
//! ```rust
//! use sluggify::sluggify:: {SluggifyOptions, sluggify};
//! fn main() {    
//!    let raw_string = r#"Welcome to the party fellas!"#;
//!
//!    let cleaned_url = sluggify(raw_string, None); 
//!
//!    println!("Sluggified version of raw string '{}' is '{}'", raw_string, cleaned_url);
//!    //Returns Sluggified version of raw string 'Welcome to the party fellas!' is 'welcome-to-the-party-fellas'
//!
//! }
//! ```
//!
//! ### You can decide to have whitespaces not trimmed or collapsed 
//!
//! ```rust
//! use sluggify::sluggify:: {SluggifyOptions, sluggify};
//! fn main() {  
//! let original = "a #$b    ** c   \t  \n  d";
//! let expected = "a---b-------c---------d";
//!
//! let options = SluggifyOptions::new().set_collapse_whitespace(false);;
//!
//! assert_eq!(sluggify(original, Some(options)), expected);
//!
//! }
//!
//! ```
//!
//! ### OR you can choose to have more control by configuring how the sluggifying happens.
//!
//!
//! ```rust
//! use sluggify::sluggify:: {SluggifyOptions, sluggify};
//! fn main() {  
//! let original = "a #$b    ** c   \t  \n  d";
//! let expected = "a-b-c-d";
//! let bad_chars = vec!['*','$', '\t'];
//!
//! let options = SluggifyOptions::new().set_collapse_whitespace(true).set_disallowed_characters(bad_chars);
//!
//! assert_eq!(sluggify(original, Some(options)), expected);
//!
//! }
//! ```






#[allow(unused_mut,unused_assignments, unused_variables)]
pub mod sluggify;
