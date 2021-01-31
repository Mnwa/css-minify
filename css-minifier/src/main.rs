use clap::Clap;
use css_minify::optimizations::{Level, Minifier};
use indoc::indoc;
use std::fs::{read_to_string, write};

#[derive(Clap)]
#[clap(version = "0.1", author = "Mnwa")]
struct Opts {
    #[clap(short, long, about = "css which will be minified")]
    input: String,
    #[clap(short, long, about = "output to optimized variant")]
    output: Option<String>,

    #[clap(
        short,
        long,
        about = indoc! {"
            Optimization levels:
                0 - Without optimizations 
                1 - Remove whitespaces, replace `0.` to `.` and others non dangerous optimizations
                2 - Level One + shortcuts (margins, paddings, backgrounds and etc). In mostly cases it's non dangerous optimizations, but be careful
                3 - Level Two + merge @media and css blocks with equal screen/selectors. It is a danger optimizations, because ordering of your css code may be changed.
        "},
        default_value = "1"
    )]
    level: Level,
}

fn main() {
    let Opts {
        input,
        output,
        level,
    } = Opts::parse();
    let mut minifier = Minifier::default();

    let input_file = read_to_string(
        shellexpand::full(&input)
            .expect("fail to parse input path")
            .to_string(),
    )
    .expect("cannot open input file");
    let minified_css = minifier.minify(&input_file, level).unwrap();

    let mut size_diff = input_file.len() - minified_css.len();
    let size_rate = ((size_diff as f64) / (input_file.len() as f64) * 100f64) as i64;
    let mut prefix = "bytes";
    if size_diff > 1024 {
        size_diff = size_diff / 1024;
        prefix = "kilobytes"
    }
    println!("You saved: {}% ({} {})", size_rate, size_diff, prefix);

    if let Some(output) = output {
        write(
            shellexpand::full(&output)
                .expect("fail to parse output path")
                .to_string(),
            minified_css,
        )
        .expect("cannot open or create the output file");
    } else {
        println!("{}", minified_css)
    }
}
