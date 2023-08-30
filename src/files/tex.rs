use crate::constants::{MAX_LODS, MAX_MIPMAPS};
use binrw::{binrw, binwrite, BinRead, BinWrite};
use bitflags::bitflags;
use ddsfile::Dds;
use std::{
    cmp::min,
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
};

use texpresso::Format;
// Attributes and Format are adapted from Lumina (https://github.com/NotAdam/Lumina/blob/master/src/Lumina/Data/Files/TexFile.cs)
bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TextureAttribute : u32 {
        const DISCARD_PER_FRAME = 0x1;
        const DISCARD_PER_MAP = 0x2;

        const MANAGED = 0x4;
        const USER_MANAGED = 0x8;
        const CPU_READ = 0x10;
        const LOCATION_MAIN = 0x20;
        const NO_GPU_READ = 0x40;
        const ALIGNED_SIZE = 0x80;
        const EDGE_CULLING = 0x100;
        const LOCATION_ONION = 0x200;
        const READ_WRITE = 0x400;
        const IMMUTABLE = 0x800;

        const TEXTURE_RENDER_TARGET = 0x100000;
        const TEXTURE_DEPTH_STENCIL = 0x200000;
        const TEXTURE_TYPE1_D = 0x400000;
        const TEXTURE_TYPE2_D = 0x800000;
        const TEXTURE_TYPE3_D = 0x1000000;
        const TEXTURE_TYPE_CUBE = 0x2000000;
        const TEXTURE_TYPE_MASK = 0x3C00000;
        const TEXTURE_SWIZZLE = 0x4000000;
        const TEXTURE_NO_TILED = 0x8000000;
        const TEXTURE_NO_SWIZZLE = 0x80000000;
    }
}

impl BinRead for TextureAttribute {
    type Args<'a> = ();
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let o = u32::read_le(reader)?;
        binrw::BinResult::Ok(TextureAttribute::from_bits(o).unwrap())
    }
}

impl BinWrite for TextureAttribute {
    type Args<'a> = ();
    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let _ = self.bits().write_le(writer);
        binrw::BinResult::Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[binrw]
#[brw(little, repr=u32)]
pub enum TextureFormat {
    B8G8R8A8 = 0x1450,
    BC1 = 0x3420,
    BC3 = 0x3431,
    BC5 = 0x6230,
    BC7 = 0x6432,
    X8R8G8B8 = 0x1451,
}

#[derive(Debug, Clone, Copy)]
#[binrw]
#[brw(little)]
pub struct TexHeader {
    pub attribute: TextureAttribute,
    pub format: TextureFormat,

    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub mip_levels: u16,

    pub lod_offsets: [u32; MAX_LODS],
    pub offset_to_surface: [u32; MAX_MIPMAPS],
}

#[derive(Debug, Clone)]
#[binwrite]
#[brw(little)]
pub struct TexFile {
    pub header: TexHeader,
    pub textures: Vec<Texture>,
    #[brw(ignore)]
    pub meta: Meta,
}

#[derive(Debug, Clone)]
#[binwrite]
#[brw(little)]
pub struct Texture {
    pub rgba: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    pub colorbytes: u32,
}

impl BinRead for TexFile {
    type Args<'a> = ();
    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut file = TexFile {
            header: TexHeader::read(reader)?,
            textures: Vec::new(),
            meta: Meta { colorbytes: 4 },
        };
        let mut texture_data_size: Vec<u32> =
            vec![0u32; min(file.header.mip_levels as usize, MAX_MIPMAPS) as usize];

        let size = texture_data_size.len();
        for i in 0..size - 1 {
            texture_data_size[i] =
                file.header.offset_to_surface[i + 1] - file.header.offset_to_surface[i];
        }
        texture_data_size[size - 1] = reader.stream_len().expect("Unable to get stream length")
            as u32
            - file.header.offset_to_surface[size - 1] as u32;

        for i in 0..file.header.mip_levels as usize {
            let mut src = vec![0u8; texture_data_size[i] as usize];
            reader
                .seek(SeekFrom::Start(file.header.offset_to_surface[i] as u64))
                .expect("Seek error");
            reader.read_exact(src.as_mut_slice())?;
            let mut size =
                (((file.header.height as u32) * (file.header.width as u32) * file.meta.colorbytes)
                    / 4u32.pow(i as u32)) as usize;
            if size == 2 {
                size = 4;
            }
            let mut dst = vec![0u8; size];
            match file.header.format {
                TextureFormat::B8G8R8A8 => {
                    dst.copy_from_slice(&src);
                }
                TextureFormat::BC1 => {
                    let format = Format::Bc1;
                    format.decompress(
                        &src,
                        file.header.width as usize,
                        file.header.height as usize,
                        dst.as_mut_slice(),
                    );
                }
                TextureFormat::BC3 => {
                    let format = Format::Bc3;
                    format.decompress(
                        &src,
                        file.header.width as usize,
                        file.header.height as usize,
                        dst.as_mut_slice(),
                    );
                }
                TextureFormat::BC5 => {
                    let format = Format::Bc5;
                    format.decompress(
                        &src,
                        file.header.width as usize,
                        file.header.height as usize,
                        dst.as_mut_slice(),
                    );
                }
                /*TextureFormat::BC7 => {
                    let format = Format::Bc7;
                    format.decompress(
                        &src,
                        file.header.width as usize,
                        file.header.height as usize,
                        dst.as_mut_slice(),
                    );
                },*/
                TextureFormat::X8R8G8B8 => {
                    dst.copy_from_slice(&src);
                }
                _ => panic!("Format unsupported"),
            }
            file.textures.push(Texture { rgba: dst })
        }
        return binrw::BinResult::Ok(file);
    }
}

impl TexFile {
    pub fn from_tex_file(filename: String) -> Option<TexFile> {
        let mut file = File::open(filename).expect("Error opening texture file");
        TexFile::read_le(&mut file).ok()
    }
    pub fn from_dds(filename: String) -> Option<TexFile> {
        let ddsfile = File::open(filename).expect("Error opening file");
        let dds = Dds::read(ddsfile).expect("Error reading dds");
        let mut txf = TexFile::new(
            dds.get_width() as u16,
            dds.get_height() as u16,
            min(dds.get_num_mipmap_levels() as u16, MAX_MIPMAPS as u16),
        );
        let mut reader = Cursor::new(dds.get_data(0).expect("Error reading layer 0"));
        txf.header.mip_levels = min(dds.get_num_mipmap_levels(), MAX_MIPMAPS as u32) as u16;
        txf.meta.colorbytes = dds.get_bits_per_pixel().unwrap_or(32) / 8;
        for i in 0..min(dds.get_num_mipmap_levels(), MAX_MIPMAPS as u32) {
            let mut tex = vec![
                0u8;
                ((dds.get_height() * dds.get_width() * txf.meta.colorbytes) / (4u32.pow(i)))
                    as usize
            ];
            reader
                .read_exact(tex.as_mut_slice())
                .expect("Error reading buffer");
            txf.push_tex(tex.as_slice());
        }
        Some(txf)
    }
    pub fn push_tex(&mut self, rgba: &[u8]) {
        let mut dst = vec![
            0u8;
            ((self.header.width as u32 * self.header.height as u32 * self.meta.colorbytes)
                / 4u32.pow(self.textures.len() as u32)) as usize
        ];
        dst.copy_from_slice(rgba);
        self.header.offset_to_surface[self.textures.len()] = if self.textures.len() <= 0 {
            80
        } else {
            self.header.offset_to_surface[self.textures.len() - 1] as u32
                + self
                    .textures
                    .get(self.textures.len() - 1)
                    .expect("No such image")
                    .rgba
                    .len() as u32
        };
        self.textures.push(Texture { rgba: dst });
    }
    pub fn write_dds(&mut self, filename: String) {
        let mut dds = Dds::new_d3d(ddsfile::NewD3dParams {
            height: self.header.height as u32,
            width: self.header.width as u32,
            depth: None,
            format: ddsfile::D3DFormat::A8R8G8B8,
            mipmap_levels: Some(min(self.header.mip_levels as u32, MAX_MIPMAPS as u32)),
            caps2: None,
        })
        .expect("error creating dds");
        let data = dds.get_mut_data(0).expect("error getting layer");
        data.copy_from_slice(
            self.textures
                .iter()
                .flat_map(|f| f.rgba.clone())
                .collect::<Vec<u8>>()
                .as_slice(),
        );
        let mut texout = File::create(filename).expect("error opening output file");
        dds.write(&mut texout).expect("error writing dds");
    }
    pub fn write_tex(&self, path: String) {
        let mut writer = File::create(path).expect("Error opening file");
        self.write(&mut writer).expect("Error writing tex");
    }

    pub fn new(width: u16, height: u16, mips: u16) -> TexFile {
        TexFile {
            header: TexHeader {
                attribute: TextureAttribute::TEXTURE_TYPE2_D,
                format: TextureFormat::B8G8R8A8,
                width: width,
                height: height,
                depth: 1,
                mip_levels: min(mips, MAX_MIPMAPS as u16),
                lod_offsets: [0, 1, 2],
                offset_to_surface: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
            textures: Vec::new(),
            meta: Meta { colorbytes: 4 },
        }
    }
}
