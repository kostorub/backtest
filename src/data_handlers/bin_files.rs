#![allow(dead_code)]
use log::debug;
use log::info;
use memmap2::Mmap;
use std::ffi::OsStr;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::data_models::be_bytes::ToFromBytes;

pub fn create_and_write_to_file<T: ToFromBytes>(
    values: &Vec<T>,
    file_path: PathBuf,
) -> io::Result<()> {
    info!("Creating file: {:?}", file_path);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path.clone())?;
    save_to_file(values, &mut file)?;
    debug!("Creating file: {:?} completed!", file_path);
    Ok(())
}

pub fn append_to_file<T: ToFromBytes>(values: &Vec<T>, file_path: PathBuf) -> io::Result<()> {
    info!("Appending to file: {:?}", file_path);
    let mut file = OpenOptions::new().append(true).open(file_path.clone())?;
    save_to_file(values, &mut file)?;
    debug!("Appending to file: {:?} completed!", file_path);
    Ok(())
}

fn save_to_file<T: ToFromBytes>(values: &Vec<T>, file: &mut File) -> io::Result<()> {
    info!("Saving to file: {:?}", file);
    for value in values {
        file.write_all(&(value.to_be_bytes()))?;
    }
    debug!("Saving to file: {:?} completed!", file);
    Ok(())
}

fn memmap_for_file(file_path: PathBuf) -> io::Result<Mmap> {
    let file = OpenOptions::new().read(true).open(file_path.clone())?;
    unsafe { Mmap::map(&file) }
}

pub fn get_values_from_file<T: ToFromBytes>(file_path: PathBuf) -> io::Result<Vec<T>> {
    let mmap = memmap_for_file(file_path)?;
    let mut result = Vec::new();
    let len = mmap.len() / T::size();
    for i in 0..len {
        result.push(T::from_be_bytes(&mmap[i * T::size()..(i + 1) * T::size()]));
    }
    Ok(result)
}

pub fn get_last_value_from_file<T: ToFromBytes>(file_path: PathBuf) -> io::Result<T> {
    let mmap = memmap_for_file(file_path)?;

    Ok(T::from_be_bytes(&mmap[mmap.len() - T::size()..mmap.len()]))
}

pub fn get_filenames(data_path: PathBuf, extension: &str) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    for entry in data_path.read_dir()? {
        let some_file = entry?;

        if some_file
            .path()
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap()
            == extension
        {
            result.push(some_file.path().to_path_buf());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::remove_file;

    use crate::{data_models::candle::Candle, tests::common::get_default_candles};

    #[test]
    fn test_create_and_write_to_file() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_1.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_append_to_file() {
        let candles: Vec<Candle> = Vec::new();
        let file_path = PathBuf::from("test_2.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();
        let candles = get_default_candles();
        append_to_file(&candles, file_path.clone()).unwrap();

        let new_candles: Vec<Candle> = get_values_from_file(file_path.clone()).unwrap();

        assert_eq!(&candles[..], &new_candles[..]);

        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_candles_from_file() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_3.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let result: Vec<Candle> = get_values_from_file(file_path.clone()).unwrap();

        assert_eq!(&result, &candles);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_last_candle_from_file() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_4.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        let candle: Candle = get_last_value_from_file(file_path.clone()).unwrap();
        let standart = get_default_candles();
        assert_eq!(Some(&candle), standart.last());
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_filenames() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("./test_5.binspecial");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let filenames = get_filenames(PathBuf::from("./"), "binspecial").unwrap();
        assert!(filenames.contains(&file_path));
        remove_file(file_path).unwrap();
    }
}
