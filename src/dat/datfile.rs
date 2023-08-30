use super::shared::{CompressionMode, DatBlockHeader, FileInfo, FileType};
use binrw::{BinRead, BinReaderExt, BinWriterExt};
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    usize,
};

pub struct DatFile {
    file: Cursor<Vec<u8>>,
}

impl DatFile {
    pub fn new(data: Vec<u8>) -> Option<DatFile> {
        Some(DatFile {
            file: Cursor::new(data),
        })
    }
    pub fn read_file(&mut self, offset: u32) -> Option<Vec<u8>> {
        let offset = offset as u64;
        self.file.set_position(offset);
        let file_info = FileInfo::read(&mut self.file).ok()?;
        let retr = match file_info.file_type {
            FileType::Empty => panic!("Invalid file"),
            FileType::Texture => self.read_texture_file(offset, &file_info),
            FileType::Standard => self.read_standard_file(offset, &file_info),
            FileType::Model => self.read_model_file(offset, &file_info),
        };
        let return_value = retr?;
        return Some(return_value);
    }

    fn read_texture_file(&mut self, offset: u64, file_info: &FileInfo) -> Option<Vec<u8>> {
        let mut data: Vec<u8> = Vec::new();

        let texture_file_info = file_info.texture_block.as_ref().unwrap();

        let mipmap_size = texture_file_info.lods[0].compressed_offset;
        if mipmap_size != 0 {
            let original_pos = self.file.position();
            self.file.set_position(offset + file_info.size as u64);
            let mut header = vec![0u8; mipmap_size as usize];
            self.file.read_exact(&mut header).ok()?;
            data.append(&mut header);
            self.file.set_position(original_pos);
        }
        for i in 0..texture_file_info.num_blocks as usize {
            let mut running_block_total =
                texture_file_info.lods[i].compressed_offset as u64 + offset + file_info.size as u64;
            data.append(&mut read_data_block(
                &mut self.file,
                running_block_total,
                true,
            )?);
            for _ in 1..texture_file_info.lods[i].block_count {
                running_block_total += self.file.read_le::<i16>().ok()? as u64;
                data.append(&mut read_data_block(
                    &mut self.file,
                    running_block_total,
                    true,
                )?);
            }
            self.file.read_le::<i16>().ok()? as u64;
        }

        return Some(data);
    }

    fn read_standard_file(&mut self, offset: u64, file_info: &FileInfo) -> Option<Vec<u8>> {
        let mut data: Vec<u8> = Vec::new();
        for block in file_info.file_block.clone().unwrap().blocks {
            data.append(&mut read_data_block(
                &mut self.file,
                file_info.size as u64 + offset + block.offset as u64,
                true,
            )?)
        }
        Some(data)
    }

    fn read_model_file(&mut self, offset: u64, file_info: &FileInfo) -> Option<Vec<u8>> {
        let mut data = Vec::new();
        data.append(&mut vec![0u8; 0x44]);

        let mdl_block = file_info.model_block.as_ref().unwrap();
        let base_offset = offset + file_info.size as u64;

        let mut total_blocks = mdl_block.num_blocks as u32;
        total_blocks += mdl_block.num.runtime_size as u32;
        for i in 0..3 {
            total_blocks += mdl_block.num.vertex[i] as u32;
        }
        for i in 0..3 {
            total_blocks += mdl_block.num.edge[i] as u32;
        }
        for i in 0..3 {
            total_blocks += mdl_block.num.index[i] as u32;
        }
        let mut compressed_block_sizes: Vec<u16> = Vec::new();
        for _ in 0..total_blocks {
            compressed_block_sizes.push(self.file.read_le::<u16>().ok()?);
        }
        let mut current_block = 0;
        let mut vert_data_off = [0i32; 3];
        let mut index_data_off = [0i32; 3];
        let mut vert_buffer_sizes = [0i32; 3];
        let mut index_buffer_sizes = [0i32; 3];

        self.file
            .set_position(base_offset + mdl_block.offset.stack_size as u64);
        //Stack
        let stack_start = data.len();
        for _ in 0..mdl_block.num.stack_size {
            let last_pos = self.file.position();
            data.append(&mut read_data_block(&mut self.file, last_pos, false)?);
            self.file
                .set_position(last_pos + compressed_block_sizes[current_block] as u64);
            current_block += 1;
        }

        let stack_end = data.len();
        let stack_size = (stack_end - stack_start) as i32;

        self.file
            .set_position(base_offset + mdl_block.offset.runtime_size as u64);
        //runtime
        let runtime_start = data.len();
        for _ in 0..mdl_block.num.runtime_size {
            let last_pos = self.file.position();
            data.append(&mut read_data_block(&mut self.file, last_pos, false)?);
            self.file
                .set_position(last_pos + compressed_block_sizes[current_block] as u64);
            current_block += 1;
        }
        let runtime_end = data.len();
        let runtime_size = (runtime_end - runtime_start) as i32;

        for i in 0..3 {
            //vertex
            if mdl_block.num.vertex[i] != 0 {
                let current_vertex_offset = data.len();
                if i == 0 || current_vertex_offset != vert_data_off[i - 1] as usize {
                    vert_data_off[i] = current_vertex_offset as i32;
                } else {
                    vert_data_off[i] = 0;
                }

                self.file
                    .set_position(base_offset + mdl_block.offset.vertex[i] as u64);

                for _ in 0..mdl_block.num.vertex[i] {
                    let last_pos = self.file.position();
                    let mut blk = read_data_block(&mut self.file, last_pos, false)?;
                    vert_buffer_sizes[i] += blk.len() as i32;
                    data.append(&mut blk);
                    self.file
                        .set_position(last_pos + compressed_block_sizes[current_block] as u64);
                    current_block += 1;
                }
            }

            //edges
            if mdl_block.num.edge[i] != 0 {
                for _ in 0..mdl_block.num.edge[i] {
                    let last_pos = self.file.position();
                    let mut blk = read_data_block(&mut self.file, last_pos, false)?;
                    data.append(&mut blk);
                    self.file
                        .set_position(last_pos + compressed_block_sizes[current_block] as u64);
                    current_block += 1;
                }
            }

            // Indexes
            if mdl_block.num.index[i] != 0 {
                let current_index_offset = data.len();

                if i == 0 || current_index_offset != index_data_off[i - 1] as usize {
                    index_data_off[i] = current_index_offset as i32
                } else {
                    index_data_off[i] = 0;
                }

                for _ in 0..mdl_block.num.index[i] {
                    let last_pos = self.file.position();
                    let mut blk = read_data_block(&mut self.file, last_pos, false)?;
                    index_buffer_sizes[i] += blk.len() as i32;
                    data.append(&mut blk);
                    self.file
                        .set_position(last_pos + compressed_block_sizes[current_block] as u64);
                    current_block += 1;
                }
            }
        }
        let mut ifo = Cursor::new(data);
        ifo.set_position(0);
        ifo.write_le::<u32>(&mdl_block.version).ok()?;
        ifo.write_le::<i32>(&stack_size).ok()?;
        ifo.write_le::<i32>(&runtime_size).ok()?;
        ifo.write_le::<u16>(&mdl_block.vertex_dec_num).ok()?;
        ifo.write_le::<u16>(&mdl_block.material_num).ok()?;
        for i in 0..3 {
            ifo.write_le::<i32>(&vert_data_off[i]).ok()?;
        }
        for i in 0..3 {
            ifo.write_le::<i32>(&index_data_off[i]).ok()?;
        }
        for i in 0..3 {
            ifo.write_le::<i32>(&vert_buffer_sizes[i]).ok()?;
        }
        for i in 0..3 {
            ifo.write_le::<i32>(&index_buffer_sizes[i]).ok()?;
        }
        ifo.write_le::<u8>(&mdl_block.num_lods).ok()?;
        ifo.write_le::<u8>(&mdl_block.index_buffer_streaming_enabled.into())
            .ok()?;
        ifo.write_le::<u8>(&mdl_block.edge_geometry_enabled.into())
            .ok()?;
        ifo.write_le::<u8>(&0u8).ok()?;
        let ret: Vec<u8> = ifo.into_inner();
        Some(ret)
    }
}

pub fn read_data_block<T: Read + Seek>(
    mut buf: T,
    starting_position: u64,
    reset_pos: bool,
) -> Option<Vec<u8>> {
    let original_pos = buf.stream_position().ok()?;

    buf.seek(SeekFrom::Start(starting_position)).ok()?;

    let block_header = DatBlockHeader::read(&mut buf).ok()?;
    let data = match block_header.compression {
        CompressionMode::Compressed {
            compressed_length,
            decompressed_length,
        } => {
            let mut compressed_data = vec![0; compressed_length as usize];
            buf.read_exact(&mut compressed_data).ok()?;

            let mut decompressed_data = vec![0; decompressed_length as usize];
            let mut decompresser = flate2::Decompress::new(false);
            match decompresser.decompress(
                compressed_data.as_slice(),
                decompressed_data.as_mut_slice(),
                flate2::FlushDecompress::Sync,
            ) {
                Err(err) => println!("{:?}", err),
                Ok(_s) => {}
            }
            decompressed_data.truncate(decompresser.total_out() as usize);
            decompressed_data
        }
        CompressionMode::Uncompressed { file_size } => {
            let mut local_data: Vec<u8> = vec![0; file_size as usize];
            buf.read_exact(&mut local_data).ok()?;
            local_data
        }
    };
    if reset_pos {
        buf.seek(SeekFrom::Start(original_pos)).ok()?;
    }
    Some(data)
}
