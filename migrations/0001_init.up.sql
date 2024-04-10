CREATE TABLE
    IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        username TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL
    );

INSERT INTO
    users (username, password)
VALUES
    (
        'admin',
        '89ba60446ddfb9f296863aaa0d6431305fa87c78058d674466672f780be9ecef'
    ) ON CONFLICT (username) DO NOTHING;

CREATE TABLE
    IF NOT EXISTS market_data (
        id INTEGER PRIMARY KEY,
        exchange TEXT NOT NULL,
        symbol TEXT NOT NULL,
        market_data_type TEXT NOT NULL,
        date_start INTEGER NOT NULL,
        date_end INTEGER NOT NULL
    );