extern crate vadeen_osm;

mod binary;
mod config;
pub mod shape;
mod sweref99tm;

use crate::config::{Config, FileConfig};
use crate::shape::{Shape, ShapeFile};
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use vadeen_osm::{Osm, OsmBuilder, Tag};

pub struct Lantmateriet {
    folder: PathBuf,
    region: String,
    config: Config,
}

const DEFAULT_CONFIG: &str = include_str!("../resources/lantmateriet_default.yml");

impl Lantmateriet {
    pub fn open<P: AsRef<Path>, C: AsRef<Path>>(
        folder: P,
        config: &Option<C>,
        region: &Option<String>,
    ) -> io::Result<Lantmateriet> {
        let region = Self::find_region(&region, &folder)?;
        let config = Self::parse_config(config)?;
        Ok(Lantmateriet {
            folder: folder.as_ref().to_path_buf(),
            region,
            config,
        })
    }

    pub fn read(self) -> io::Result<Osm> {
        let mut osm = OsmBuilder::default();
        for file_def in &self.config.files {
            if let Err(e) = self.read_shape_file(&mut osm, &file_def) {
                if let ErrorKind::NotFound = e.kind() {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
        Ok(osm.build())
    }

    fn find_region<P: AsRef<Path>>(region: &Option<String>, path: P) -> io::Result<String> {
        if let Some(region) = region {
            return Ok(region.clone());
        }

        let file_names: Vec<String> = fs::read_dir(path)?
            .map(|e| {
                // Turns a DirEntry into the file name. (Maybe a better way?)
                e.as_ref()
                    .unwrap()
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .filter(|s| s.matches('_').count() == 1)
            .take(1)
            .collect();
        let split: Vec<&str> = file_names[0].split('_').collect();
        Ok((*split.last().unwrap()).to_owned())
    }

    fn parse_config<P: AsRef<Path>>(path: &Option<P>) -> io::Result<Config> {
        if let Some(path) = path {
            Ok(Config::open(path)?)
        } else {
            Ok(Config::parse_string(DEFAULT_CONFIG)?)
        }
    }

    fn read_shape_file(&self, osm: &mut OsmBuilder, file_def: &FileConfig) -> io::Result<()> {
        let base_name = format!("{}_{}", file_def.name, &self.region);
        let shape = ShapeFile::open(&self.folder, &base_name)?;

        for record in shape {
            let kkod = record.attributes.get("KKOD").unwrap();

            if let Some(conf) = file_def.kkods.get(kkod) {
                let mut tags: Vec<Tag> = conf.tags();
                tags.push(("kkod".to_owned(), kkod.to_owned()).into());
                tags.push(("layer".to_owned(), file_def.name.to_owned()).into());

                match record.shape {
                    Shape::Point(c) => {
                        osm.add_point(c, tags.clone());
                    }
                    Shape::PolyLine(poly) => {
                        for points in poly.parts {
                            osm.add_polyline(points, tags.clone());
                        }
                    }
                    Shape::Polygon(poly) => {
                        osm.add_polygon(poly.parts, tags);
                    }
                }
            }
        }
        Ok(())
    }
}
