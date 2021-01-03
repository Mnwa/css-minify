use clap::Clap;
use css_minify_lib::optimizations::Minifier;
use std::fs::{read_to_string, write};

#[derive(Clap)]
#[clap(version = "1.0", author = "Mnwa")]
struct Opts {
    #[clap(short, long, about = "css which will be minified")]
    input: String,
    #[clap(short, long, about = "output to optimized variant")]
    output: Option<String>,
}

fn main() {
    let Opts { input, output } = Opts::parse();
    let mut minifier = Minifier::default();

    let input_file = read_to_string(
        shellexpand::full(&input)
            .expect("fail to parse input path")
            .to_string(),
    )
    .expect("cannot open input file");
    let (other_input, minified_css) = minifier.minify(&input_file).unwrap();

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
