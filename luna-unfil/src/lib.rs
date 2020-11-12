use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

const SECTOR_SIZE: u32 = 2048;

#[derive(Debug)]
pub struct FilHeader {
    pub num_files: u32,
    pub file_size: u32,
}

#[derive(Debug)]
pub struct FilEntry {
    pub path: String,
    pub sector: u32,
    pub size: u32,
    pub created: u32,
}

#[derive(Debug)]
pub struct Fil {
    pub header: FilHeader,
    pub entries: Vec<FilEntry>,
    pub buffer: Vec<u8>,
}

impl Fil {
    pub fn new() -> Fil {
        Fil {
            header: FilHeader {
                num_files: 0,
                file_size: 0,
            },
            entries: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Fil {
        let mut cursor = Cursor::new(buffer);

        cursor.seek(SeekFrom::Start(20)).unwrap();

        let num_files = cursor.read_u32::<LittleEndian>().unwrap();
        let file_size = cursor.read_u32::<LittleEndian>().unwrap();

        let header = FilHeader {
            num_files,
            file_size,
        };

        cursor.seek(SeekFrom::Current(4)).unwrap();

        let entries = (1..num_files)
            .map(|_| {
                let mut path_buffer = [0; 20];
                cursor.read_exact(&mut path_buffer).unwrap();

                let path = String::from_utf8(Vec::from(path_buffer))
                    .unwrap()
                    .trim_matches(char::from(0))
                    .to_string();

                let sector = cursor.read_u32::<LittleEndian>().unwrap();
                let size = cursor.read_u32::<LittleEndian>().unwrap();
                let created = cursor.read_u32::<LittleEndian>().unwrap();

                FilEntry {
                    path,
                    sector,
                    size,
                    created,
                }
            })
            .collect();

        Fil {
            header,
            entries,
            buffer: cursor.into_inner(),
        }
    }

    pub fn read_entry(&self, entry: &FilEntry) -> &[u8] {
        let start = entry.sector * SECTOR_SIZE;
        let end = start + entry.size;

        &self.buffer[start as usize..end as usize]
    }
}
