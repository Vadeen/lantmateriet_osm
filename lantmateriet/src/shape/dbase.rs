use crate::binary::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug)]
pub struct DBase {
    reader: BufReader<File>,
    header: Header,
}

#[derive(Debug)]
pub struct Record {
    pub fields: HashMap<String, String>,
}

#[derive(Debug)]
struct Header {
    record_count: u32,
    record_size: u16,
    field_descriptors: Vec<FieldDescriptor>,
}

#[derive(Debug)]
struct FieldDescriptor {
    field_name: String,
    field_type: char,
    field_size: u8,
    field_decimal_count: u8,
}

impl Iterator for DBase {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        let mut fields: HashMap<String, String> = HashMap::new();

        let prefix = read_u8(&mut self.reader).unwrap();
        if prefix == 0x1A {
            return None;
        }

        for fd in &self.header.field_descriptors {
            let value = Self::read_string(&mut self.reader, fd.field_size.into()).unwrap();
            fields.insert(fd.field_name.clone(), value);
        }

        Some(Record { fields })
    }
}

impl DBase {
    pub fn open(file: File) -> io::Result<DBase> {
        let mut reader = BufReader::new(file);
        let header = Self::read_header(&mut reader)?;

        Ok(DBase { reader, header })
    }

    fn read_header(reader: &mut (impl Read + Seek)) -> io::Result<Header> {
        // Ignore version, table, last changed etc...
        reader.seek(SeekFrom::Current(4))?;

        let record_count = read_le_u32(reader)?;
        let header_size = read_le_u16(reader)?;
        let record_size = read_le_u16(reader)?;

        // Ignore incomplete transactions, encryption, mdx etc...
        reader.seek(SeekFrom::Current(20))?;

        let field_count = (header_size - 32 - 1) / 32;
        let field_descriptors = Self::read_field_descriptors(reader, field_count)?;

        Ok(Header {
            record_count,
            record_size,
            field_descriptors,
        })
    }

    fn read_field_descriptors(
        reader: &mut (impl Read + Seek),
        fields: u16,
    ) -> io::Result<Vec<FieldDescriptor>> {
        let mut descriptors = Vec::with_capacity(fields.into());
        for _ in 0..fields {
            descriptors.push(Self::read_field_descriptor(reader)?);
        }

        let terminator = read_u8(reader)?;
        if terminator != 0x0D {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Expected field descriptor terminator 0x0D but found 0x{:02X}",
                    terminator
                ),
            ));
        }

        Ok(descriptors)
    }

    fn read_field_descriptor(reader: &mut (impl Read + Seek)) -> io::Result<FieldDescriptor> {
        let field_name = Self::read_string(reader, 11)?;
        let field_type = read_char(reader)?;

        // Reserved 4 bytes
        reader.seek(SeekFrom::Current(4))?;

        let field_size = read_u8(reader)?;
        let field_decimal_count = read_u8(reader)?;

        // We don't care about work area id and such.
        reader.seek(SeekFrom::Current(14))?;

        Ok(FieldDescriptor {
            field_name,
            field_type,
            field_size,
            field_decimal_count,
        })
    }

    fn read_string(reader: &mut impl Read, size: u64) -> io::Result<String> {
        let mut bytes = Vec::new();
        reader.take(size).read_to_end(&mut bytes)?;
        let string: String = bytes
            .iter()
            .take_while(|&c| c != &0)
            .map(|&c| c as char)
            .collect();
        Ok(string.trim().to_owned())
    }
}
