# sluggify-rs
Simple slug or clean url generator for Rust.

With new settings, you will get an hyphenized, lowercase, alphanumeric version of any string you please, with any diacritics removed, whitespace and dashes collapsed, and whitespace trimmed. If you are looking for a new feature or
notice a bug open a PR or issue on the [Github repository](https://github.com/propenster/sluggify-rs) and I'll try
to get to it as quickly as possible.

[github]: https://github.com/propenster/sluggify-rs

Please see the [API documentation](https://docs.rs/sluggify) for available features and examples of how to use them.

 # Getting started

 We explain how to use sluggify step-by-step.
 Users who already have experience with Rust can skip right to [Step 3: Use sluggify in your project](https://docs.rs/sluggify/#step-3-use-sluggify-in-your-project).
 Users who already know `sluggify` might want to jump right into the [modules docs](https://docs.rs/sluggify/#modules)

 ## Step 1: Setting up Rust

 Rust can be installed following the instruction for [rustup](https://rustup.rs/).


 ## Step 2: Setting up a new Rust project

 Since sluggify is a library, you need to setup your own new Rust project to use sluggify.
 With Rust, projects and their dependencies are managed with the builtin package manager [Cargo](https://doc.rust-lang.org/cargo/index.html).
 To create a new Rust project, issue

 ```bash
 cargo new hello_world --bin
 cd hello_world
 ```
 in your terminal. The flag `--bin` tells Cargo to create an executable project instead of a library.
 In [this section](https://doc.rust-lang.org/nightly/book/hello-cargo.html#a-new-project) of the Rust docs, you find details about what Cargo just created for you.

 Your new project can be compiled with
 ```bash
 cargo build
 ```
 If dependencies in your project are out of date, update with
 ```bash
 cargo update
 ```
 Execute the compiled code with
 ```bash
 cargo run
 ```
 If you are new to Rust, we suggest to proceed with [learning Rust](https://www.rust-lang.org/learn) via the Rust docs.

 ## Step 3: Use sluggify in your project

 To use sluggify in your Rust project, add the following to your `Cargo.toml`

 ```toml
 [dependencies]
 sluggify = "0.1.0"
 ```


## Quick Start

```rust

use sluggify::sluggify:: {SluggifyOptions, sluggify};

// Print a web page onto stdout
fn main() {    
    let raw_string = "Welcome to the party fellas!";

    let cleaned_url = sluggify(raw_string, None); 

    println!("Sluggified version of raw string '{}' is '{}'", raw_string, cleaned_url);
    Returns ``` r#Sluggified version of raw string 'Welcome to the party fellas!' is 'welcome-to-the-party-fellas'r# ```

}
```

### Use sluggify with withspace configuration
```rust

use sluggify::sluggify:: {SluggifyOptions, sluggify};

// Print a web page onto stdout
//this replaces all whitespace characters with a hyphen '-'
fn main() {    
    let options = SluggifyOptions::new().set_collapse_whitespace(false); //it is true by new

    let raw_string = "a #$b    ** c   \t  \n  d ! ... xyz...23";

    let cleaned_url = sluggify(raw_string, Some(options)); 

    println!("Sluggified version of raw string {} is {}", raw_string, cleaned_url);
}

```

### Use sluggify by overiding forbidden characters
```rust

use sluggify::sluggify:: {SluggifyOptions, sluggify};

// Print a web page onto stdout
fn main() {    
    let raw_string = "a #$b    ** c   \t  \n  d";
    let bad_characters = vec!['*','$', '\t'];

    let options = SluggifyOptions::new().set_disallowed_characters(bad_characters);

    let cleaned_url = sluggify(raw_string, Some(options)); 

    println!("Sluggified version of raw string {} is {}", raw_string, cleaned_url);
}

```

### Configure all options at the same time
```rust

use sluggify::sluggify:: {SluggifyOptions, sluggify};

// Print a web page onto stdout
fn main() {    
    let raw_string = "a #$b    ** c   \t  \n  d";
    let bad_characters = vec!['*','$', '\t'];

    //don't collapse whitespaces and also look out for these bad characters...
    let options = SluggifyOptions::new().set_collapse_whitespace(false).set_disallowed_characters(chars);

    //pass this option to the sluggify helper
    let cleaned_url = sluggify(raw_string, Some(options)); 

    println!("Sluggified version of raw string {} is {}", raw_string, cleaned_url);
}

```


## Changelog

0.1.0:
 - Initial release.


## License
```
MIT License

Copyright (c) 2023 propenster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```