use clap::Parser;

fn main() {
    let opts = playground_rust::opts::Opts::parse();
    println!("{:?}", opts);
}
