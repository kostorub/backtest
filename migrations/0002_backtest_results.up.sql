-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS backtest_results (
        id INTEGER PRIMARY KEY,
        metrics_id INTEGER NOT NULL,
        symbol TEXT NOT NULL,
        exchange TEXT NOT NULL,
        market_data_type TEXT NOT NULL,
        chart_market_data_type TEXT NOT NULL,
        date_start TEXT NOT NULL,
        date_end TEXT NOT NULL,
        deposit REAL NOT NULL,
        commission REAL NOT NULL,
        price_low REAL NOT NULL,
        price_high REAL NOT NULL,
        grid_count INTEGER NOT NULL,
        grid_trigger REAL NOT NULL,
        grid_sl REAL,
        grid_tp REAL,
        sell_all BOOLEAN NOT NULL,
        positions BLOB NOT NULL,
        FOREIGN KEY (metrics_id) REFERENCES metrics (id) ON UPDATE CASCADE ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS metrics (
        id INTEGER PRIMARY KEY,
        positions_number INTEGER NOT NULL,
        profit_positions_number INTEGER NOT NULL,
        profit_positions_percent REAL NOT NULL,
        loss_positions_number INTEGER NOT NULL,
        loss_positions_percent REAL NOT NULL,
        average_profit_position REAL NOT NULL,
        average_loss_position REAL NOT NULL,
        number_of_currency u32 profit_per_position_in_percent REAL NOT NULL,
        profit_factor REAL NOT NULL,
        expected_payoff REAL NOT NULL,
        sortino REAL NOT NULL,
        average_position_size REAL NOT NULL,
        start_deposit REAL NOT NULL,
        finish_deposit REAL NOT NULL,
        total_profit REAL NOT NULL,
        total_profit_percent REAL NOT NULL,
        max_deposit REAL NOT NULL,
        max_drawdown REAL NOT NULL,
        drawdown REAL NOT NULL,
        max_use_of_funds REAL NOT NULL
    );