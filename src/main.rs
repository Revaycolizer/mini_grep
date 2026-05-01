use clap::Parser;

mod tools;

#[derive(Parser)]
#[command(name="mini_grep")]
#[command(about="Search for a string in a file",long_about=None)]

struct Args{
    needle:String,
    haystack:String,
}

fn main() {

    let args = Args::parse();

    tools::grep::search(&args.needle, &args.haystack);

}
