
use byteorder::{LittleEndian, ReadBytesExt};
use ark_std::io::{Read, Seek, SeekFrom};
use ark_serialize::{SerializationError};
use ark_serialize::SerializationError::IoError;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};


type IoResult<T> = Result<T, SerializationError>;


pub struct BinFile<R: Read + Seek> {
    pub reader: R,
    pub bin_type: String,
    pub sections_map: HashMap::<u32, Vec<Section>>,
    pub reading_section: Option<Section>
}

pub struct Section {
    pub offset: u32,
    pub size: u32
}

impl<R: Read + Seek> BinFile<R> {
    pub fn new_from_reader(mut reader: R, bin_type: String, max_version: u32) -> IoResult<BinFile<R>> {
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;

        if bin_type != String::from_utf8(header.to_vec()).unwrap() {
            return Err(IoError(Error::new(
                ErrorKind::InvalidData,
                "Invalid magic number",
            )));
        }

        let version = reader.read_u32::<LittleEndian>()?;
        if version > max_version {
            return Err(IoError(Error::new(
                ErrorKind::InvalidData,
                "Invalid version number",
            )));
        }

        let n_sections = reader.read_u32::<LittleEndian>()?;

        let mut sec_map = HashMap::<u32, Vec<Section>>::new();

        for _ in 0..n_sections {
            let sec_type = reader.read_u32::<LittleEndian>()?;
            let sec_size = reader.read_u64::<LittleEndian>()?;
            let offset = reader.stream_position()?;
            if let Some(sec) = sec_map.get_mut(&sec_type) {
                sec.push(Section { offset: offset as u32, size: sec_size as u32 });
            } else {
                sec_map.insert(sec_type, vec![Section { offset: offset as u32, size: sec_size as u32 }]);
            }
           
            reader.seek(SeekFrom::Current(sec_size as i64))?;
        }
        reader.seek(SeekFrom::Start(0))?;

        Ok(BinFile {
            reader,
            bin_type,
            sections_map: sec_map,
            reading_section: None
        })
    }

    pub fn read_u32_le(&mut self) -> u32 {
        self.reader.read_u32::<LittleEndian>().unwrap()
    }

    pub fn read_u64_le(&mut self) -> u64 {
        self.reader.read_u64::<LittleEndian>().unwrap()
    }

    pub fn start_read_section(&mut self, section_id: u32, section_index: Option<u32>) -> IoResult<()>{
        let section = self.sections_map.get(&section_id).ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                "No section offset for wire2label type found",)
            }).unwrap();
        let section_index: u32 = if (section_index.is_some() && section_index.unwrap() < section.len() as u32) {
            section_index.unwrap()
        } else { 0 };
        let section = section.get(section_index as usize).unwrap();
        self.reading_section = Some(Section { offset: section.offset, size: section.size });
        self.reader.seek(SeekFrom::Start(section.offset as u64 + section.size as u64)).unwrap();
        Ok(())
    }

    pub fn end_read_section(&mut self, check: Option<bool>) -> IoResult<()> {
        
        self.reading_section = None;
        Ok(())
    }
}

