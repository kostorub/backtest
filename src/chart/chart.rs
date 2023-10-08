use std::path::PathBuf;

use chrono::offset::TimeZone;
use chrono::{DateTime, Duration, Utc};
use plotters::prelude::*;

use crate::backtest::strategies::grid::settings::GridSettingsRequest;
use crate::data_models::market_data::kline::KLine;
use crate::data_models::market_data::position::Position;

fn parse_time(t: u64) -> DateTime<Utc> {
    Utc.timestamp_opt((t / 1000) as i64, 0).unwrap()
}

pub fn build_chart(
    settings: &GridSettingsRequest,
    klines: Vec<KLine>,
    positions: &Vec<Position>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.svg", settings.backtest_uuid);
    let filepath = PathBuf::from("src/web/static/img/").join(&filename);
    let root = SVGBackend::new(&filepath, (1920, 1440)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(20, 20, 20, 20);

    let (to_date, from_date) = (
        parse_time(klines.first().unwrap().date) + Duration::days(1),
        parse_time(klines.last().unwrap().date) - Duration::days(1),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(70)
        .build_cartesian_2d(to_date..from_date, settings.price_low..settings.price_high)?;

    chart
        .configure_mesh()
        .x_labels(settings.grids_count as usize)
        .light_line_style(&WHITE)
        .draw()?;

    chart.draw_series(klines.iter().map(|k| {
        CandleStick::new(
            parse_time(k.date),
            k.open,
            k.high,
            k.low,
            k.close,
            GREEN.filled(),
            RED,
            5,
        )
    }))?;

    for pos in positions {
        chart.draw_series(LineSeries::new(
            vec![
                (parse_time(pos.open_date()), pos.open_price()),
                (parse_time(pos.last_date()), pos.last_price()),
            ],
            &BLUE,
        ))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    // root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filepath.to_str().unwrap());

    Ok(())
}
