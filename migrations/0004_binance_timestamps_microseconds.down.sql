UPDATE market_data
SET
    date_start = date_start / 1000,
    date_end = date_end / 1000
WHERE date_start >= 100000000000000;

UPDATE backtest_data
SET
    date_start = date_start / 1000,
    date_end = date_end / 1000
WHERE date_start >= 100000000000000;
