use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use build_const::{src_file, ConstWriter};
use css_minify::optimizations::{Level, Minifier};

fn main() {
    let main = src_file!("styles.rs");
    let consts = ConstWriter::from_path(Path::new(main)).unwrap();
    let mut consts = consts.finish_dependencies();
    let mut file = File::open(current_dir().unwrap().join("static/main.css")).unwrap();
    let mut styles = String::new();
    file.read_to_string(&mut styles).unwrap();
    consts.add_value("STYLES", "&str", Minifier::default().minify(styles.as_str(), Level::Three).unwrap());
}