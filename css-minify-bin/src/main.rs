use clap::Clap;
use css_minify_lib::optimizations::{Level, Minifier};
use indoc::indoc;
use std::fs::{read_to_string, write};

#[derive(Clap)]
#[clap(version = "1.0", author = "Mnwa")]
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
    let (other_input, minified_css) = minifier.minify(&input_file, level).unwrap();

    if let Some(output) = output {
        if !other_input.is_empty() {
            println!(
                "There is chunk of content which was not parsed: \n{}",
                other_input
            )
        }

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
