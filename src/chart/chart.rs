use chrono::offset::{Local, TimeZone};
use chrono::{DateTime, Duration, Utc};
use plotters::prelude::*;

use crate::data_models::market_data::kline::KLine;

fn parse_time(t: u64) -> DateTime<Utc> {
    Utc.timestamp_opt(t as i64, 0).unwrap()
}

const OUT_FILE_NAME: &'static str = "stock.png";

pub fn build_chart(klines: Vec<KLine>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (4096, 2160)).into_drawing_area();
    root.fill(&WHITE)?;

    let (to_date, from_date) = (
        parse_time(klines.first().unwrap().date) + Duration::days(1),
        parse_time(klines.last().unwrap().date) - Duration::days(1),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("KLines", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(from_date..to_date, 0f64..40000f64)?;

    chart.configure_mesh().light_line_style(&WHITE).draw()?;

    chart.draw_series(klines.iter().map(|k| {
        CandleStick::new(
            parse_time(k.date),
            k.open,
            k.high,
            k.low,
            k.close,
            GREEN.filled(),
            RED,
            3,
        )
    }))?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    // root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
