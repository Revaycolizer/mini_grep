use clap::Parser;

mod tools;

#[derive(Parser)]
#[command(name = "mini_grep")]
#[command(about="Search for a string in a file",long_about=None)]

struct Args {
    needle: String,
    haystack: String,
    #[arg(short = 'i', long)]
    ignore_case: bool,
    #[arg(short = 'n', long)]
    line_numbers: bool,
}

fn main() {
    let args = Args::parse();

    tools::grep::search(
        &args.needle,
        &args.haystack,
        args.ignore_case,
        args.line_numbers,
    );
}
