use build_const::{src_file, ConstWriter};
use css_minify::optimizations::{Level, Minifier};
use std::env::current_dir;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Read;
use std::path::Path;

fn main() {
    let main = src_file!("styles.rs");
    let consts = ConstWriter::from_path(Path::new(main)).unwrap();
    let mut consts = consts.finish_dependencies();
    let mut file = File::open(current_dir().unwrap().join("static/main.css")).unwrap();
    let mut styles = String::new();
    let mut hasher = DefaultHasher::default();
    styles.hash(&mut hasher);
    let hash = hasher.finish().to_string();
    file.read_to_string(&mut styles).unwrap();
    consts.add_value(
        "STYLES",
        "&str",
        Minifier::default()
            .minify(styles.as_str(), Level::Three)
            .unwrap(),
    );
    consts.add_value("STYLES_HASH", "&str", hash);
}
