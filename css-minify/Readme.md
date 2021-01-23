# CSS minify
[![](https://github.com/Mnwa/css-minify/workflows/css-minify/badge.svg?branch=master)](https://github.com/Mnwa/css-minify/actions?query=workflow%3Acss-minify)
[![](https://github.com/Mnwa/css-minify/workflows/clippy/badge.svg?branch=master)](https://github.com/Mnwa/css-minify/actions?query=workflow%3Aclippy)
[![](https://docs.rs/css-minify/badge.svg)](https://docs.rs/css-minify/)
[![](https://img.shields.io/crates/v/css-minify.svg)](https://crates.io/crates/css-minify)
[![](https://img.shields.io/crates/d/css-minify.svg)](https://crates.io/crates/css-minify)

CSS minification library based on `nom`.
This library parses css input, minifies it and applies some level-dependent optimizations to it.

```rust
use css_minify::optimizations::{Minifier, Level};
fn main() {
    assert_eq!(
        Minifier::default().minify(
            r#"
                 #some_id, input {
                     padding: 5px 3px; /* Mega comment */
                     color: white;
                 }
                 
                 
                 /* this is are test id */
                 #some_id_2, .class {
                     padding: 5px 4px; /* Mega comment */
                     Color: rgb(255, 255, 255);
                 }
             "#,
            Level::Three
        ),
        Ok("#some_id,input{color:white;padding:5px 3px}#some_id_2,.class{color:#fff;padding:5px 4px}".into())
    )
}
```

# CSS minifier
[![](https://github.com/Mnwa/css-minifier/workflows/css-minifier/badge.svg?branch=master)](https://github.com/Mnwa/css-minifier/actions?query=workflow%3Acss-minifier)
[![](https://docs.rs/css-minifier/badge.svg)](https://docs.rs/css-minifier/)
[![](https://img.shields.io/crates/v/css-minifier.svg)](https://crates.io/crates/css-minifier)
[![](https://img.shields.io/crates/d/css-minifier.svg)](https://crates.io/crates/css-minifier)

CLI wrapper for css-minify library.

```
css-minifier 0.1

USAGE:
    css-minifier [OPTIONS] --input <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>      css which will be minified
    -l, --level <level>      Optimization levels:
                                 0 - Without optimizations 
                                 1 - Remove whitespaces, replace `0.` to `.` and others non
                             dangerous optimizations
                                 2 - Level One + shortcuts (margins, paddings, backgrounds and etc).
                             In mostly cases it's non dangerous optimizations, but be careful
                                 3 - Level Two + merge @media and css blocks with equal
                             screen/selectors. It is a danger optimizations, because ordering of
                             your css code may be changed.
                              [default: 1]
    -o, --output <output>    output to optimized variant
```

Example
```bash
css-minifier -i "~/test.css" -o "~/test.min.css" -l 3
```