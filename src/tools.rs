use crate::files::tex::TexFile;
use ddsfile::Dds;
use std::fs::File;

pub fn dds_tex(ddsfile: String, tex: String) {
    let txf = TexFile::from_dds(ddsfile).expect("Error reading dds");
    txf.write_tex(tex);
}

pub fn scale_extract(upper: String, lower: String, output: String) {
    let upper_file = File::open(upper).expect("Scales layer cannot open");
    let lower_file = File::open(lower).expect("Base layer cannot open");
    let mut udds = Dds::read(upper_file).expect("Unable to read scale layer");
    let ldds = Dds::read(lower_file).expect("Unable to read base-texture");
    let mut out_file = File::create(output).expect("error creating output");
    for i in (0..udds.data.len()).step_by(4) {
        if udds.data[i] == ldds.data[i]
            && udds.data[i + 1] == ldds.data[i + 1]
            && udds.data[i + 2] == ldds.data[i + 2]
            && udds.data[i + 3] == ldds.data[i + 3]
        {
            udds.data[i] = 0;
            udds.data[i + 1] = 0;
            udds.data[i + 2] = 0;
            udds.data[i + 3] = 0;
        }
    }
    udds.write(&mut out_file)
        .expect("error writing new dds file");
}

pub fn tex_dds(tex: String, dds_out: String) {
    let mut tex = TexFile::from_tex_file(tex).expect("Error reading tex");
    tex.write_dds(dds_out)
}
