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

-- Assign all roles to the User 1
INSERT OR IGNORE INTO users_roles (user_id, role_id) VALUES (1, 1), (1, 2), (1, 3), (1, 4), (1, 5);

CREATE TABLE
    IF NOT EXISTS market_data (
        id INTEGER PRIMARY KEY,
        exchange TEXT NOT NULL,
        symbol TEXT NOT NULL,
        market_data_type TEXT NOT NULL,
        date_start INTEGER NOT NULL,
        date_end INTEGER NOT NULL
    );
