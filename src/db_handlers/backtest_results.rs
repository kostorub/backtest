use sqlx::{Error, Pool, Sqlite};

use crate::{
    backtest::{settings::BacktestSettings, strategies::grid::settings::GridSettingsRequest},
    data_models::{
        backtest_results::BacktestResults,
        market_data::{metrics::Metrics, position::Position},
    },
};

pub async fn insert_backtest_metrics(metrics: &Metrics, pool: &Pool<Sqlite>) -> Result<i64, Error> {
    let positions_number = metrics.positions_number as i64;
    let profit_positions_number = metrics.profit_positions_number as i64;
    let loss_positions_number = metrics.loss_positions_number as i64;

    let result = sqlx::query!(
        "INSERT INTO metrics (
            positions_number,
            profit_positions_number,
            profit_positions_percent,
            loss_positions_number,
            loss_positions_percent,
            average_profit_position,
            average_loss_position,
            number_of_currency,
            profit_factor,
            expected_payoff,
            sortino,
            average_position_size,
            start_deposit,
            finish_deposit,
            total_profit,
            total_profit_percent,
            max_deposit,
            max_drawdown,
            drawdown,
            max_use_of_funds
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20
        )",
        positions_number,
        profit_positions_number,
        metrics.profit_positions_percent,
        loss_positions_number,
        metrics.loss_positions_percent,
        metrics.average_profit_position,
        metrics.average_loss_position,
        metrics.number_of_currency,
        metrics.profit_factor,
        metrics.expected_payoff,
        metrics.sortino,
        metrics.average_position_size,
        metrics.start_deposit,
        metrics.finish_deposit,
        metrics.total_profit,
        metrics.total_profit_percent,
        metrics.max_deposit,
        metrics.max_drawdown,
        metrics.drawdown,
        metrics.max_use_of_funds
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn insert_backtest_results(
    backtest_settings: &BacktestSettings,
    grid_settings: &GridSettingsRequest,
    positions: &Vec<Position>,
    metrics_id: i64,
    pool: &Pool<Sqlite>,
) -> Result<i64, Error> {
    let market_data_type = backtest_settings.market_data_type.value().0;
    let chart_market_data_type = grid_settings.chart_market_data_type.value().0;
    let date_start = grid_settings.date_start.clone();
    let date_end = grid_settings.date_end.clone();
    let grids_count = grid_settings.grids_count as i64;
    let positions = serde_json::to_string(&positions).unwrap();

    let result = sqlx::query!(
        "INSERT INTO backtest_results (
            metrics_id,
            symbol,
            exchange,
            market_data_type,
            chart_market_data_type,
            date_start,
            date_end,
            deposit,
            commission,
            price_low,
            price_high,
            grid_count,
            grid_trigger,
            grid_sl,
            grid_tp,
            sell_all,
            positions
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17
        )",
        metrics_id,
        backtest_settings.symbols[0],
        backtest_settings.exchange,
        market_data_type,
        chart_market_data_type,
        date_start,
        date_end,
        backtest_settings.deposit,
        backtest_settings.commission,
        grid_settings.price_low,
        grid_settings.price_high,
        grids_count,
        grid_settings.grid_trigger,
        grid_settings.grid_sl,
        grid_settings.grid_tp,
        grid_settings.sell_all,
        positions
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_backtest_results(
    backtest_results_id: i64,
    pool: &Pool<Sqlite>,
) -> Result<BacktestResults, Error> {
    let row = sqlx::query!(
        "SELECT * FROM backtest_results WHERE id = ?1",
        backtest_results_id
    )
    .fetch_one(pool)
    .await?;

    let result = BacktestResults {
        id: row.id,
        metrics_id: row.metrics_id,
        symbol: row.symbol,
        exchange: row.exchange,
        market_data_type: row.market_data_type.into(),
        chart_market_data_type: row.chart_market_data_type.into(),
        date_start: row.date_start,
        date_end: row.date_end,
        deposit: row.deposit,
        commission: row.commission,
        price_low: row.price_low,
        price_high: row.price_high,
        grid_count: row.grid_count,
        grid_trigger: row.grid_trigger,
        grid_sl: row.grid_sl,
        grid_tp: row.grid_tp,
        sell_all: Some(row.sell_all),
        positions: serde_json::from_str(&row.positions).unwrap(),
    };

    Ok(result)
}

pub async fn get_backtest_metrics(
    backtest_results_id: i64,
    pool: &Pool<Sqlite>,
) -> Result<Metrics, Error> {
    let row = sqlx::query!(
        "SELECT metrics.* FROM metrics JOIN backtest_results ON backtest_results.metrics_id = metrics.id WHERE backtest_results.id = ?1",
        backtest_results_id
    )
    .fetch_one(pool)
    .await?;

    let result = Metrics {
        id: row.id,
        positions_number: row.positions_number as u64,
        profit_positions_number: row.profit_positions_number as u64,
        profit_positions_percent: row.profit_positions_percent,
        loss_positions_number: row.loss_positions_number as u64,
        loss_positions_percent: row.loss_positions_percent,
        average_profit_position: row.average_profit_position,
        average_loss_position: row.average_loss_position,
        number_of_currency: row.number_of_currency as u32,
        profit_per_position_in_percent: row.profit_per_position_in_percent,
        profit_factor: row.profit_factor,
        expected_payoff: row.expected_payoff,
        sortino: row.sortino,
        average_position_size: row.average_position_size,
        start_deposit: row.start_deposit,
        finish_deposit: row.finish_deposit,
        total_profit: row.total_profit,
        total_profit_percent: row.total_profit_percent,
        max_deposit: row.max_deposit,
        max_drawdown: row.max_drawdown,
        drawdown: row.drawdown,
        max_use_of_funds: row.max_use_of_funds,
    };

    Ok(result)
}
