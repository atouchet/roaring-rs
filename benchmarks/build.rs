use std::fs::File;
use std::io::{Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

use bytes::Bytes;
use convert_case::{Case, Casing};
use reqwest::{blocking::get, IntoUrl};
use zip::read::ZipArchive;

const BASE_URL: &str = "https://github.com/RoaringBitmap/real-roaring-datasets/raw/master/";

// const DATASET_CENSUS_INCOME: &str = "census-income";
// const DATASET_CENSUS_INCOME_SRT: &str = "census-income_srt";
// const DATASET_CENSUS1881: &str = "census1881";
// const DATASET_CENSUS1881_SRT: &str = "census1881_srt";
// const DATASET_DIMENSION_003: &str = "dimension_003";
// const DATASET_DIMENSION_008: &str = "dimension_008";
// const DATASET_DIMENSION_033: &str = "dimension_033";
// const DATASET_USCENSUS2000: &str = "uscensus2000";
// const DATASET_WEATHER_SEPT_85: &str = "weather_sept_85";
// const DATASET_WEATHER_SEPT_85_SRT: &str = "weather_sept_85_srt";
// const DATASET_WIKILEAKS_NOQUOTES: &str = "wikileaks-noquotes";
const DATASET_WIKILEAKS_NOQUOTES_SRT: &str = "wikileaks-noquotes_srt";

// const DATASETS: &[&str] = &[
//     DATASET_CENSUS_INCOME,
//     DATASET_CENSUS_INCOME_SRT,
//     DATASET_CENSUS1881,
//     DATASET_CENSUS1881_SRT,
//     DATASET_DIMENSION_003,
//     DATASET_DIMENSION_008,
//     DATASET_DIMENSION_033,
//     DATASET_USCENSUS2000,
//     DATASET_WEATHER_SEPT_85,
//     DATASET_WEATHER_SEPT_85_SRT,
//     DATASET_WIKILEAKS_NOQUOTES,
//     DATASET_WIKILEAKS_NOQUOTES_SRT,
// ];

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let benches_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("benches");
    let mut manifest_paths_file = File::create(benches_dir.join("datasets_paths.rs"))?;

    writeln!(
        &mut manifest_paths_file,
        "// This file is generated by the build script.\n// Do not modify by hand, use the build.rs file.\n"
    )?;

    for dataset in &[DATASET_WIKILEAKS_NOQUOTES_SRT] {
        let out_path = out_dir.join(dataset);
        let url = format!("{}/{}.zip", BASE_URL, dataset);
        let bytes = download_dataset(url)?;
        unzip_in_folder(bytes, &out_path)?;

        writeln!(
            &mut manifest_paths_file,
            r#"pub const {}: &str = {:?};"#,
            dataset.to_case(Case::ScreamingSnake),
            out_path.display(),
        )?;
    }

    Ok(())
}

fn download_dataset<U: IntoUrl>(url: U) -> anyhow::Result<Cursor<Bytes>> {
    let bytes = get(url)?.bytes()?;
    Ok(Cursor::new(bytes))
}

fn unzip_in_folder<R: Read + Seek, P: AsRef<Path>>(bytes: R, path: P) -> anyhow::Result<()> {
    let path = path.as_ref();
    fs::create_dir_all(path).unwrap();
    let mut zip = ZipArchive::new(bytes)?;
    zip.extract(path)?;
    Ok(())
}