use rand::seq::SliceRandom;
use serde::{de, Deserialize, Deserializer};

use crate::{filename, full_path, nixinfo::NixInfo};
use std::{collections::HashMap, fs, path::PathBuf};

pub fn dir() -> PathBuf {
    full_path("~/Pictures/Wallpapers")
}

pub fn current() -> Option<String> {
    let curr = NixInfo::after().wallpaper;

    let wallpaper = {
        if curr == "./foo/bar.text" {
            fs::read_to_string(
                dirs::runtime_dir()
                    .expect("could not get $XDG_RUNTIME_DIR")
                    .join("current_wallpaper"),
            )
            .ok()
        } else {
            Some(curr)
        }
    };

    Some(
        wallpaper
            .expect("no wallpaper found")
            .replace("/persist", ""),
    )
}

fn filter_images<P>(dir: P) -> impl Iterator<Item = String>
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    dir.as_ref()
        .read_dir()
        .unwrap_or_else(|_| panic!("could not read {:?}", &dir))
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    return matches!(ext.to_str(), Some("jpg" | "jpeg" | "png" | "webp")).then(
                        || {
                            path.to_str()
                                .expect("could not convert path to str")
                                .to_string()
                        },
                    );
                }
            }

            None
        })
}

/// returns all files in the wallpaper directory, exlcluding the current wallpaper
pub fn all() -> Vec<String> {
    let curr = self::current().unwrap_or_default();

    filter_images(&self::dir())
        // do not include the current wallpaper
        .filter(|path| curr != *path)
        .collect()
}

pub fn random() -> String {
    if self::dir().exists() {
        self::all()
            .choose(&mut rand::thread_rng())
            // use fallback image if not available
            .unwrap_or(&NixInfo::before().fallback)
            .to_string()
    } else {
        NixInfo::before().fallback
    }
}

pub fn random_from_dir<P>(dir: P) -> String
where
    P: AsRef<std::path::Path> + std::fmt::Debug,
{
    filter_images(dir)
        .collect::<Vec<_>>()
        .choose(&mut rand::thread_rng())
        // use fallback image if not available
        .unwrap_or(&NixInfo::before().fallback)
        .to_string()
}

/// reads the wallpaper info from wallpapers.csv
pub fn get_wallpaper_info(image: &String) -> Option<WallInfo> {
    let wallpapers_csv = full_path("~/Pictures/Wallpapers/wallpapers.csv");
    if !wallpapers_csv.exists() {
        return None;
    }

    let reader = std::io::BufReader::new(
        std::fs::File::open(wallpapers_csv).expect("could not open wallpapers.csv"),
    );

    let fname = filename(image);
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.deserialize::<WallInfo>()
        .flatten()
        .find(|line| line.filename == fname)
}

/// euclid's algorithm to find the greatest common divisor
const fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

#[derive(Debug, Clone)]
pub struct WallInfo {
    pub filename: String,
    pub width: u32,
    pub height: u32,
    pub geometries: HashMap<String, String>,
    pub wallust: String,
}

impl WallInfo {
    pub fn get_geometry(&self, width: i32, height: i32) -> Option<(f64, f64, f64, f64)> {
        self.get_geometry_str(width, height).and_then(|geom| {
            let geometry: Vec<_> = geom.split(|c| c == '+' || c == 'x').collect();
            let w: f64 = geometry[0].parse().ok()?;
            let h: f64 = geometry[1].parse().ok()?;
            let x: f64 = geometry[2].parse().ok()?;
            let y: f64 = geometry[3].parse().ok()?;

            Some((w, h, x, y))
        })
    }

    pub fn get_geometry_str(&self, width: i32, height: i32) -> Option<&String> {
        let divisor = gcd(width, height);
        self.geometries
            .get(&format!("{}x{}", width / divisor, height / divisor))
    }
}

impl<'de> Deserialize<'de> for WallInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Filename,
            Faces,
            Geometries,
            Wallust,
        }

        struct WallInfoVisitor;

        impl<'de> de::Visitor<'de> for WallInfoVisitor {
            type Value = WallInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct WallInfo2")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut filename = None;
                let mut width = None;
                let mut height = None;
                let mut geometries = HashMap::new();
                let mut wallust = None;

                while let Some((key, value)) = map.next_entry::<&str, String>()? {
                    match key {
                        "filename" => {
                            filename = Some(value);
                        }
                        "width" => {
                            width = Some(value.parse::<u32>().map_err(de::Error::custom)?);
                        }
                        "height" => {
                            height = Some(value.parse::<u32>().map_err(de::Error::custom)?);
                        }
                        // ignore
                        "faces" => {}
                        "wallust" => {
                            wallust = Some(value);
                        }
                        _ => {
                            geometries.insert(key.to_string(), value);
                        }
                    }
                }

                let filename = filename.ok_or_else(|| de::Error::missing_field("filename"))?;
                let width = width.ok_or_else(|| de::Error::missing_field("width"))?;
                let height = height.ok_or_else(|| de::Error::missing_field("height"))?;
                let wallust = wallust.ok_or_else(|| de::Error::missing_field("wallust"))?;

                // geometries have no width and height, calculate from wall info
                Ok(WallInfo {
                    filename,
                    width,
                    height,
                    geometries,
                    wallust,
                })
            }
        }

        const FIELDS: &[&str] = &[
            "filename",
            "width",
            "height",
            "faces",
            "geometries",
            "wallust",
        ];
        deserializer.deserialize_struct("WallInfo", FIELDS, WallInfoVisitor)
    }
}
