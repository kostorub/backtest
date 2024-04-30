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
use crate::data_models::market_data::enums::MarketDataType;
use crate::data_models::market_data::kline_trait::KLineTrait;

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

pub fn get_values_from_file<T: ToFromBytes + KLineTrait>(
    file_path: PathBuf,
    date_start: i64,
    date_end: i64,
    mdt: MarketDataType,
) -> io::Result<Vec<T>> {
    let first_value = get_first_value_from_file::<T>(file_path.clone())?;
    let last_value = get_last_value_from_file::<T>(file_path.clone())?;

    // If the requested date range is outside the range of the file, return an empty vector
    if date_start > last_value.date() || date_end < first_value.date() {
        return Ok(Vec::new());
    }

    let mut offset = 0 as usize;
    if date_start > first_value.date() {
        offset = ((date_start - first_value.date()) / mdt.value().1) as usize;
    };

    let mmap = memmap_for_file(file_path)?;

    let mut len = mmap.len() / T::size() - offset;
    if date_end < last_value.date() {
        len = len - ((last_value.date() - date_end) / mdt.value().1) as usize;
    }

    let mut result = Vec::new();

    for i in 0..len {
        let v = T::from_be_bytes(&mmap[(offset + i) * T::size()..(offset + i + 1) * T::size()]);
        if v.date() >= date_start && v.date() <= date_end {
            result.push(v);
        }
    }
    Ok(result)
}

pub fn get_first_value_from_file<T: ToFromBytes>(file_path: PathBuf) -> io::Result<T> {
    let mmap = memmap_for_file(file_path)?;

    Ok(T::from_be_bytes(&mmap[0..T::size()]))
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

pub fn bin_file_name(exchange: String, symbol: String, market_data_type: MarketDataType) -> String {
    format!(
        "{}-{}-{}.marketdata",
        exchange,
        symbol,
        market_data_type.value().0
    )
    .to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::remove_file;

    use crate::{data_models::market_data::kline::KLine, tests::common::get_default_candles};

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
        let candles: Vec<KLine> = Vec::new();
        let file_path = PathBuf::from("test_2.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();
        let candles = get_default_candles();
        append_to_file(&candles, file_path.clone()).unwrap();

        let new_candles: Vec<KLine> = get_values_from_file(
            file_path.clone(),
            candles.first().unwrap().date(),
            candles.last().unwrap().date(),
            MarketDataType::KLine1m,
        )
        .unwrap();

        assert_eq!(&candles[..], &new_candles[..]);

        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_candles_from_file() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_3.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let result: Vec<KLine> =
            get_values_from_file(file_path.clone(), 0, i64::MAX, MarketDataType::KLine1m).unwrap();

        assert_eq!(&result, &candles);
        remove_file(file_path).unwrap();
    }

    /// [] - actual data
    /// {} - requested data
    /// {[}] - current test
    #[test]
    fn test_get_candles_from_file_1() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_4.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let result: Vec<KLine> = get_values_from_file(
            file_path.clone(),
            candles.first().unwrap().date() - MarketDataType::KLine1m.value().1,
            candles.last().unwrap().date() - MarketDataType::KLine1m.value().1,
            MarketDataType::KLine1m,
        )
        .unwrap();

        assert_eq!(&result, &candles[..candles.len() - 1]);
        remove_file(file_path).unwrap();
    }

    /// [] - actual data
    /// {} - requested data
    /// [{]} - current test
    #[test]
    fn test_get_candles_from_file_2() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_5.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let result: Vec<KLine> = get_values_from_file(
            file_path.clone(),
            candles.first().unwrap().date() + MarketDataType::KLine1m.value().1,
            candles.last().unwrap().date() + MarketDataType::KLine1m.value().1,
            MarketDataType::KLine1m,
        )
        .unwrap();

        assert_eq!(&result, &candles[1..]);
        remove_file(file_path).unwrap();
    }

    /// [] - actual data
    /// {} - requested data
    /// [{]} - current test
    #[test]
    fn test_get_candles_from_file_3() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_6.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let result: Vec<KLine> = get_values_from_file(
            file_path.clone(),
            candles.first().unwrap().date() + MarketDataType::KLine1m.value().1,
            candles.last().unwrap().date() + MarketDataType::KLine1m.value().1,
            MarketDataType::KLine1m,
        )
        .unwrap();

        assert_eq!(&result, &candles[1..]);
        remove_file(file_path).unwrap();
    }

    /// [] - actual data
    /// {} - requested data
    /// [{}] - current test
    #[test]
    fn test_get_candles_from_file_4() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_7.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let result: Vec<KLine> = get_values_from_file(
            file_path.clone(),
            candles.first().unwrap().date() + MarketDataType::KLine1m.value().1,
            candles.last().unwrap().date() - MarketDataType::KLine1m.value().1,
            MarketDataType::KLine1m,
        )
        .unwrap();

        assert_eq!(&result, &candles[1..candles.len() - 1]);
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_first_value_from_file() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_8.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        let candle: KLine = get_first_value_from_file(file_path.clone()).unwrap();
        let standart = get_default_candles();
        assert_eq!(Some(&candle), standart.first());
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_last_value_from_file() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("test_9.bin");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        let candle: KLine = get_last_value_from_file(file_path.clone()).unwrap();
        let standart = get_default_candles();
        assert_eq!(Some(&candle), standart.last());
        remove_file(file_path).unwrap();
    }

    #[test]
    fn test_get_filenames() {
        let candles = get_default_candles();
        let file_path = PathBuf::from("./test_10.binspecial");
        create_and_write_to_file(&candles, file_path.clone()).unwrap();

        assert!(file_path.exists());

        let filenames = get_filenames(PathBuf::from("./"), "binspecial").unwrap();
        assert!(filenames.contains(&file_path));
        remove_file(file_path).unwrap();
    }
}
