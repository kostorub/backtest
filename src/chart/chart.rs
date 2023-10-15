use std::path::PathBuf;

use chrono::offset::TimeZone;
use chrono::Utc;

use plotly::color::NamedColor;
use plotly::common::Line;
use plotly::{Candlestick, Layout, Plot, Scatter};
use std::fs;

use crate::backtest::strategies::grid::settings::GridSettingsRequest;
use crate::data_models::market_data::enums::MarketDataType;
use crate::data_models::market_data::kline::KLine;
use crate::data_models::market_data::position::Position;

fn parse_time(t: u64, mdt: MarketDataType) -> String {
    Utc.timestamp_opt((t / mdt.value().1 * mdt.value().1) as i64 / 1000, 0)
        .unwrap()
        .to_string()
}

pub fn build_chart(
    s: &GridSettingsRequest,
    klines: Vec<KLine>,
    positions: &Vec<Position>,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.html", s.backtest_uuid);
    let filepath = PathBuf::from("src/web/static/charts/").join(&filename);

    let mut plot = Plot::new();

    let layout = Layout::new().height(800);
    plot.set_layout(layout);

    let x = klines
        .iter()
        .map(|k| parse_time(k.date, s.chart_market_data_type.clone()))
        .collect::<Vec<_>>();
    let open = klines.iter().map(|k| k.open).collect::<Vec<_>>();
    let high = klines.iter().map(|k| k.high).collect::<Vec<_>>();
    let low = klines.iter().map(|k| k.low).collect::<Vec<_>>();
    let close = klines.iter().map(|k| k.close).collect::<Vec<_>>();

    let trace = Box::new(Candlestick::new(x, open, high, low, close).show_legend(false));
    plot.add_trace(trace);

    for pos in positions {
        let mut sc = Scatter::new(
            vec![
                parse_time(pos.open_date(), s.chart_market_data_type.clone()),
                parse_time(pos.last_date(), s.chart_market_data_type.clone()),
            ],
            vec![pos.open_price(), pos.last_price()],
        );
        if pos.pnl.unwrap() > 0.0 {
            let line = Line::new().color(NamedColor::Green);
            sc = sc.line(line);
            sc = sc.show_legend(false);
        } else {
            let line = Line::new().color(NamedColor::Red);
            sc = sc.line(line);
            sc = sc.show_legend(false);
        }
        plot.add_trace(sc);
    }

    let plot_result = plot.to_inline_html(None);
    fs::write(&filepath, plot_result).unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    // root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filepath.to_str().unwrap());

    Ok(())
}
