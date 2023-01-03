use crate::{meta::MetaFileHeader, pesqpack::DatFile, structs::ExtendedModPack, tex::TexFile};
use binrw::BinRead;
use std::{
    fs::{File, OpenOptions},
    io::{Cursor, Read, Write},
};
use zip::{read::ZipFile, ZipArchive};

pub fn dds_tex(ddsfile: String, tex: String) {
    let txf = TexFile::from_dds(ddsfile).expect("Error reading DDSFile");
    txf.write_tex(tex);
}

pub fn tex_dds(tex: String, dds_out: String) {
    let mut tex = TexFile::from_tex_file(tex).expect("Error reading tex");
    tex.write_dds(dds_out)
}

pub fn export_ttmp(path: String, _output: String) {
    let fil = File::open(path).expect("Unable to open ttmp");
    let mut zip = ZipArchive::new(fil).unwrap();
    let emp = serde_json::from_reader::<ZipFile, ExtendedModPack>(
        zip.by_name("TTMPL.mpl")
            .expect("Could not extract modpack json"),
    )
    .expect("Modpack parsing error");
    let mut md = Vec::new();
    zip.by_name("TTMPD.mpd")
        .expect("Could not extract modpack data")
        .read_to_end(&mut md)
        .expect("Error readiung zipped mod data");
    let mut dat = DatFile::new(md).expect("Error opening dat");
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.tex")
        .expect("Error reading file")
        .write_all(
            dat.read_file(emp.ModPackPages[12].ModGroups[2].OptionList[0].ModsJsons[1].ModOffset)
                .expect("error reading file")
                .as_slice(),
        )
        .expect("Error writing file");
    println!(
        "{:?}",
        emp.ModPackPages[12].ModGroups[2].OptionList[0].ModsJsons[1].FullPath
    );
}

pub fn parse_meta_file(path: String) {
    let mut file = File::open(path).expect("Error opening file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Error reading file");
    parse_meta(buf);
}

fn parse_meta(meta: Vec<u8>) {
    let mut parser = Cursor::new(meta);
    let meta = MetaFileHeader::read(&mut parser).expect("Error parsing meta block");
    let mta = meta.parse_meta_blocks(parser);
    println!("{:?}", mta);
    println!("{:?}", meta);
    println!(
        "{}",
        serde_json::to_string(&mta).expect("error converting to json")
    );
}
