#![feature(seek_stream_len, cursor_remaining)]
mod constants;
mod dat;
mod files;
mod interop;
mod meta;
mod structs;
mod tools;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Commands {
    Export { output: String, ttmp: String },
    Dds2tex { input: String, output: String },
    Tex2dds { input: String, output: String },
    ParseMeta { input: String },
}

fn main() {
    let opt = Commands::from_args();
    println!("{:?}", opt);
    match opt {
        Commands::Export { output, ttmp } => tools::export_ttmp(ttmp, output),
        Commands::Dds2tex { input, output } => {
            tools::dds_tex(input, output);
        }
        Commands::Tex2dds { input, output } => tools::tex_dds(input, output),
        Commands::ParseMeta { input } => {
            tools::parse_meta_file(input);
        }
    }
}
