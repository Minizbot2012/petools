use crate::{
    dat::datfile::DatFile,
    files::tex::TexFile,
    meta::metaparser::{MetaFileHeader, MetaManipulation},
    structs::ExtendedModPack,
};
use binrw::BinRead;
use std::{
    fs::{self, File},
    io::{Cursor, Read, Write},
    path::PathBuf,
};
use zip_next::{read::ZipFile, ZipArchive};

pub fn dds_tex(ddsfile: String, tex: String) {
    let txf = TexFile::from_dds(ddsfile).expect("Error reading dds");
    txf.write_tex(tex);
}

pub fn tex_dds(tex: String, dds_out: String) {
    let mut tex = TexFile::from_tex_file(tex).expect("Error reading tex");
    tex.write_dds(dds_out)
}

pub fn export_ttmp(path: String, output: String) {
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
        .expect("Error reading zipped mod data");
    let dat = &mut DatFile::new(md).expect("Error opening dat");
    let mut root = PathBuf::from(output);
    root.push(emp.Name);
    if root.exists() {
        fs::remove_dir_all(root.clone()).expect("Error removing directory");
    }
    if emp.SimpleModsList.is_some() {
        for bmp in emp.SimpleModsList.unwrap() {
            extract(
                dat,
                bmp.ModOffset,
                PathBuf::from(bmp.FullPath),
                root.clone(),
            );
        }
    }
    if emp.ModPackPages.is_some() {
        for page in emp.ModPackPages.unwrap() {
            root.push(format!("{}", page.PageIndex));
            for group in page.ModGroups {
                root.push(group.GroupName);
                for option in group.OptionList {
                    root.push(option.GroupName);
                    for files in option.ModsJsons {
                        extract(
                            dat,
                            files.ModOffset,
                            PathBuf::from(files.FullPath),
                            root.clone(),
                        )
                    }
                    root.pop();
                }
                root.pop();
            }
            root.pop();
        }
    }
}

fn extract(dat: &mut DatFile, offset: u32, fp: PathBuf, root: PathBuf) {
    let data = dat.read_file(offset).expect("Error reading data");
    println!("Extracting {:?}", fp);
    let jp = root.join(fp);
    if jp.extension().unwrap() == "meta" {
        //parse meta
        parse_meta(data);
    } else {
        //write file
        fs::create_dir_all(jp.parent().expect("Path has no parent"))
            .expect("error creating folder");
        let mut file = File::create(jp).expect("Error opening file");
        file.write(data.as_slice()).expect("Error writing file");
    }
}

pub fn parse_meta_file(path: String) {
    let mut file = File::open(path).expect("Error opening file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Error reading file");
    parse_meta(buf);
}

fn parse_meta(meta: Vec<u8>) -> Vec<MetaManipulation> {
    let mut parser = Cursor::new(meta);
    let mut meta = MetaFileHeader::read(&mut parser).expect("Error parsing meta block");
    meta.parse_meta_blocks(parser)
}
