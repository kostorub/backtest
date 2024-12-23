CREATE TABLE
    IF NOT EXISTS users (
        user_id INTEGER PRIMARY KEY,                    -- Unique ID for each user
        account_number TEXT NOT NULL UNIQUE,            -- Unique hashed account number for each user
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP  -- Timestamp of when the user was created
    );

CREATE TABLE
    IF NOT EXISTS roles (
        role_id INTEGER PRIMARY KEY,    -- Unique ID for each role
        role_name TEXT NOT NULL UNIQUE, -- Unique name for each role (e.g., 'MarketDataViewer', 'GridBacktestRunner')
        description TEXT                -- Description of the role (optional)
    );

CREATE TABLE
    IF NOT EXISTS users_roles (
        user_id INTEGER NOT NULL,               -- Foreign key referencing users table
        role_id INTEGER NOT NULL,               -- Foreign key referencing roles table
        PRIMARY KEY (user_id, role_id),         -- Composite primary key
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
        FOREIGN KEY (role_id) REFERENCES roles(role_id) ON DELETE CASCADE
    );

-- Add a user
INSERT INTO users (account_number) VALUES ('7044898b8da316331e51648494daa910e4a90ed3ae269b3beaca09fa43be28b2') ON CONFLICT (account_number) DO NOTHING;

-- Add roles
INSERT INTO roles 
        (role_name, description) 
    VALUES 
        ('MarketDataViewer', 'Allows viewing the market data page'),        -- Role 1
        ('MarketDataEditor', 'Allows editing the market data page'),        -- Role 2
        ('GridBacktestViewer', 'Allows viewing the grid backtest page'),    -- Role 3
        ('GridBacktestRunner', 'Allows running grid backtests'),            -- Role 4
        ('GridBacktestTrialRunner', 'Allows running grid backtest in trial version'); -- Role 5

-- Assign all roles to the User 1 except the GridBacktestTrialRunner role
INSERT OR IGNORE INTO users_roles (user_id, role_id) VALUES (1, 1), (1, 2), (1, 3), (1, 4);

CREATE TABLE
    IF NOT EXISTS market_data (
        id INTEGER PRIMARY KEY,
        exchange TEXT NOT NULL,
        symbol TEXT NOT NULL,
        market_data_type TEXT NOT NULL,
        date_start INTEGER NOT NULL,
        date_end INTEGER NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS common_backtest (
        common_id INTEGER PRIMARY KEY,
        user_id INTEGER NOT NULL,
        symbol TEXT NOT NULL,
        exchange TEXT NOT NULL,
        market_data_type TEXT NOT NULL,
        chart_market_data_type TEXT NOT NULL,
        date_start INTEGER NOT NULL,
        date_end INTEGER NOT NULL,
        deposit REAL NOT NULL,
        commission REAL NOT NULL,
        positions TEXT NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (user_id) ON UPDATE CASCADE ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS grid_backtest (
        grid_id INTEGER PRIMARY KEY,
        common_id INTEGER NOT NULL,
        price_low REAL NOT NULL,
        price_high REAL NOT NULL,
        grid_count INTEGER NOT NULL,
        grid_trigger REAL NOT NULL,
        grid_sl REAL,
        grid_tp REAL,
        sell_all BOOLEAN NOT NULL,
        FOREIGN KEY (common_id) REFERENCES common_backtest (common_id) ON UPDATE CASCADE ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS trailing_backtest (
        trailing_id INTEGER PRIMARY KEY,
        common_id INTEGER NOT NULL,
        bounce_off_buy REAL NOT NULL,
        bounce_off_sell REAL NOT NULL,
        min_tp REAL NOT NULL,
        sl REAL NOT NULL,
        FOREIGN KEY (common_id) REFERENCES common_backtest (common_id) ON UPDATE CASCADE ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS backtest_metrics (
        id INTEGER PRIMARY KEY,
        backtest_id INTEGER NOT NULL,
        positions_number INTEGER NOT NULL,
        profit_positions_number INTEGER NOT NULL,
        profit_positions_percent REAL NOT NULL,
        loss_positions_number INTEGER NOT NULL,
        loss_positions_percent REAL NOT NULL,
        average_profit_position REAL NOT NULL,
        average_loss_position REAL NOT NULL,
        number_of_currency INTEGER NOT NULL,
        profit_per_position_in_percent REAL NOT NULL,
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
        max_use_of_funds REAL NOT NULL,
        FOREIGN KEY (backtest_id) REFERENCES common_backtest (common_id) ON UPDATE CASCADE ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS key_value_store (
    key TEXT PRIMARY KEY,            -- Unique key
    value TEXT NOT NULL,             -- Value associated with the key
    expires_at TIMESTAMP             -- Expiration timestamp
);