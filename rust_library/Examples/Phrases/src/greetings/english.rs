//! This module contains English phrases.
//!
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!",
//!   phrases::greetings::english::hello(),
//!   username);
//! ```

/*
 whatever
 */

/// Applies to code that follows it.
/// In this case, it's our `hello()` function.
pub fn hello() -> String { return "hello".to_string(); /* this is easy */ }
pub fn goodbye() -> String { return "goobye".to_string(); }