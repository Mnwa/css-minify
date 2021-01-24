//! CSS minification library based on `nom`
//! This library parses css input, minifies it and applies some level-dependent optimizations to it.
//!
//! ```rust
//! use css_minify::optimizations::{Minifier, Level};
//! fn main() {
//!     assert_eq!(
//!         Minifier::default().minify(
//!             r#"
//!                  #some_id, input {
//!                      padding: 5px 3px; /* Mega comment */
//!                      color: white;
//!                  }
//!                  
//!                  
//!                  /* this is are test id */
//!                  #some_id_2, .class {
//!                      padding: 5px 4px; /* Mega comment */
//!                      Color: rgb(255, 255, 255);
//!                  }
//!              "#,
//!              Level::Three
//!         ),
//!         Ok("#some_id,input{padding:5px 3px;color:white}#some_id_2,.class{padding:5px 4px;color:#fff}".into())
//!     )
//! }
//! ```

pub mod optimizations;
pub(crate) mod parsers;
pub(crate) mod structure;
