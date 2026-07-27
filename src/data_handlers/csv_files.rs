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

#[cfg(test)]
mod tests {
    use std::fs::{remove_file, write};

    use crate::data_models::market_data::kline::KLine;

    use super::load_data_from_csv;

    #[test]
    fn binance_kline_row_preserves_microsecond_open_time() {
        let path =
            std::env::temp_dir().join(format!("backtest-binance-kline-{}.csv", std::process::id()));
        write(
            &path,
            "1784764800000000,66079.36000000,66096.56000000,66076.41000000,66088.26000000,3.18765000,1784764859999999,210648.88503810,276,1.86484000,123231.36103590,0\n",
        )
        .unwrap();

        let klines = load_data_from_csv::<KLine>(path.clone());

        assert_eq!(klines.len(), 1);
        assert_eq!(klines[0].date, 1_784_764_800_000_000);
        assert_eq!(klines[0].open, 66_079.36);
        assert_eq!(klines[0].high, 66_096.56);
        assert_eq!(klines[0].low, 66_076.41);
        assert_eq!(klines[0].close, 66_088.26);
        assert_eq!(klines[0].volume, 3.18765);

        remove_file(path).unwrap();
    }
}
