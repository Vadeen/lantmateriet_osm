mod dbase;

use crate::binary::*;
use crate::shape::dbase::DBase;
use crate::sweref99tm::to_wgs;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;
use vadeen_osm::geo::{Boundary, Coordinate};

#[derive(Debug)]
pub struct ShapeFile {
    reader: BufReader<File>,
    pub header: Header,
    dbase: DBase,
}

#[derive(Debug)]
pub struct Record {
    pub shape: Shape,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Shape {
    Point(Coordinate),
    PolyLine(Poly),
    Polygon(Poly),
}

#[derive(Debug)]
pub struct Poly {
    bounds: Boundary,
    pub parts: Vec<Vec<Coordinate>>,
}

#[derive(Debug)]
pub struct Header {
    shape_type: u8,
    pub bounds: Boundary,
}

impl Iterator for ShapeFile {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        self.dbase.next().map(|r| Record {
            shape: self.read_shape().unwrap(),
            attributes: r.fields,
        })
    }
}

impl ShapeFile {
    pub fn open<P: AsRef<Path>>(path: P, base_name: &str) -> io::Result<ShapeFile> {
        let dbase_file = Self::open_file(path.as_ref(), base_name, ".dbf")?;
        let shape_file = Self::open_file(path.as_ref(), base_name, ".shp")?;

        let dbase = DBase::open(dbase_file)?;
        let mut reader = BufReader::new(shape_file);

        let header = Self::read_header(&mut reader)?;

        Ok(ShapeFile {
            reader,
            header,
            dbase,
        })
    }

    fn read_header(reader: &mut (impl Read + Seek)) -> io::Result<Header> {
        // Skip a lot for now.
        reader.seek(SeekFrom::Current(32))?;

        let shape_type = read_le_u32(reader)? as u8;
        let bounds = Self::read_mbr(reader)?;

        // Skip a lot for now.
        reader.seek(SeekFrom::Current(32))?;

        Ok(Header { bounds, shape_type })
    }

    fn open_file(base_path: &Path, base_name: &str, ext: &str) -> io::Result<File> {
        let file_name = base_name.to_owned() + ext;
        let mut file_path = base_path.to_path_buf();
        file_path.push(file_name);

        match File::open(&file_path) {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => Err(io::Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Could not open shape file {:?}, file not found.",
                        &file_path
                    ),
                )),
                _ => Err(e),
            },
            ok => ok,
        }
    }

    fn read_shape(&mut self) -> io::Result<Shape> {
        let _rec_num = read_be_u32(&mut self.reader)?;
        let _rec_len = read_be_u32(&mut self.reader)?;
        let rec_type = read_le_u32(&mut self.reader)?;

        match rec_type {
            1 => Ok(Shape::Point(self.read_point()?)),
            3 => Ok(Shape::PolyLine(self.read_poly()?)),
            5 => Ok(Shape::Polygon(self.read_poly()?)),
            _ => Err(io::Error::new(
                ErrorKind::Other,
                format!("Unknown shp record type: {}", rec_type),
            )),
        }
    }

    fn read_poly(&mut self) -> io::Result<Poly> {
        let bounds = Self::read_mbr(&mut self.reader)?;
        let part_count = read_le_u32(&mut self.reader)?;
        let point_count = read_le_u32(&mut self.reader)?;

        let mut part_indexes = Vec::new();
        for _ in 0..part_count {
            part_indexes.push(read_le_u32(&mut self.reader)?);
        }

        let mut i = 0;
        let mut parts = Vec::new();
        for idx in part_indexes.iter().skip(1) {
            let mut part = Vec::new();

            for _ in i..*idx {
                part.push(self.read_point()?);
            }

            i = *idx;
            parts.push(part);
        }

        let mut part = Vec::new();
        for _ in i..point_count {
            part.push(self.read_point()?);
        }
        parts.push(part);

        Ok(Poly { bounds, parts })
    }

    fn read_point(&mut self) -> io::Result<Coordinate> {
        let x = read_le_f64(&mut self.reader)?;
        let y = read_le_f64(&mut self.reader)?;
        Ok(to_wgs(y, x))
    }

    fn read_mbr(reader: &mut (impl Read + Seek)) -> io::Result<Boundary> {
        let min_x = read_le_f64(reader)?;
        let min_y = read_le_f64(reader)?;
        let max_x = read_le_f64(reader)?;
        let max_y = read_le_f64(reader)?;

        let min = to_wgs(min_y, min_x);
        let max = to_wgs(max_y, max_x);

        Ok(Boundary {
            min,
            max,
            freeze: false,
        })
    }
}
