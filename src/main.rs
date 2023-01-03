#![feature(seek_stream_len, cursor_remaining)]
use structopt::StructOpt;
mod meta;
mod tools;
mod structs;
mod tex;
mod pesqpack;
mod gamedata;
mod models;
mod macros;

#[derive(Debug, StructOpt)]
enum Commands {
    Export { ttmp: String, output: String },
    Dds2tex { input: String, output: String },
    Tex2dds { input: String, output: String },
    ParseMeta { input: String },
}

fn main() {
    let opt = Commands::from_args();
    match opt {
        Commands::Export { ttmp, output } => {
            tools::export_ttmp(ttmp, output)
        }
        Commands::Dds2tex { input, output } => {
            tools::dds_tex(input, output);
        }
        Commands::Tex2dds { input, output } => {
            tools::tex_dds(input, output)
        }
        Commands::ParseMeta { input } => {
            tools::parse_meta_file(input);
        }
    }
}
