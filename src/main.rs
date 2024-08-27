#![feature(seek_stream_len)]
mod constants;
mod files;
mod tools;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Commands {
    Dds2tex {
        input: String,
        output: String,
    },
    Tex2dds {
        input: String,
        output: String,
    },
    ScaleExtract {
        upper: String,
        lower: String,
        out: String,
    },
}

fn main() {
    let opt = Commands::from_args();
    println!("{:?}", opt);
    match opt {
        Commands::Dds2tex { input, output } => {
            tools::dds_tex(input, output);
        }
        Commands::Tex2dds { input, output } => tools::tex_dds(input, output),
        Commands::ScaleExtract { upper, lower, out } => tools::scale_extract(upper, lower, out),
    }
}
