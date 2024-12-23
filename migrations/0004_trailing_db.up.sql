-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS trailing_data (
        id INTEGER PRIMARY KEY,
        metrics_id INTEGER NOT NULL,
        symbol TEXT NOT NULL,
        exchange TEXT NOT NULL,
        market_data_type TEXT NOT NULL,
        chart_market_data_type TEXT NOT NULL,
        date_start INTEGER NOT NULL,
        date_end INTEGER NOT NULL,
        deposit REAL NOT NULL,
        commission REAL NOT NULL,
        bounce_off_buy REAL NOT NULL,
        bounce_off_sell REAL NOT NULL,
        min_tp REAL NOT NULL,
        sl REAL NOT NULL,
        positions TEXT NOT NULL,
        FOREIGN KEY (metrics_id) REFERENCES backtest_metrics (id) ON UPDATE CASCADE ON DELETE CASCADE
    );

-- rename the backtest_data table to the grid_data table
ALTER TABLE backtest_data RENAME TO grid_data;