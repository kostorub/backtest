use std::path::PathBuf;

use csv::ReaderBuilder;
use log::{debug, info, warn};
use serde::de::DeserializeOwned;

pub fn load_data_from_csv<T>(csv_path: PathBuf) -> Vec<T>
where
    T: DeserializeOwned,
{
    info!("Loading data from csv: {:?}", csv_path);
    let mut result = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_path(&csv_path)
        .unwrap_or_else(|_| panic!("Can't read the file {}", csv_path.to_string_lossy()));
    for raw_value in rdr.deserialize() {
        match raw_value {
            Ok(value) => {
                let trade: T = value;
                // if trade.timestamp > start_date && trade.timestamp < end_date {
                result.push(trade);
                // }
            }
            Err(e) => {
                warn!("Error deserializing trade, check csv file format: {:?}", e);
            }
        };
    }
    debug!("Loading data from csv: {:?} completed!", csv_path);
    result
}
