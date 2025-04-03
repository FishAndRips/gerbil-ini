#![no_std]

//! # gerbil-ini
//!
//! Simple no-std compatible .ini parsing library.
//!
//! Example usage
//!
//! ```rust
//! use gerbil_ini::{Ini, IniMode};
//!
//! let some_ini = r#"
//! ; This is a comment.
//!
//! [My Section]
//! ; Here's a value
//! some KEY=This is a value!
//!
//! ; Here's another value
//! anotherkey=This is yet another value!
//!
//! ;commented out=this value isn't real :(
//!
//! [Another Section]
//! yourkey=This is a value!
//! some KEY=This, too, is a value!
//! anotherkey=//Wow Look At Me I'm A Value\\
//! "#;
//!
//! let ini = Ini::parse(some_ini, IniMode::Simple).expect("parse");
//! let section = ini.get_section("My Section").unwrap();
//! assert_eq!(section.get("some KEY"), Some("This is a value!"));
//! ```

extern crate alloc;

mod ini;
pub use ini::*;

