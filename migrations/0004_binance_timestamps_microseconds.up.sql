-- Binance candle files use Unix epoch microseconds. Convert legacy metadata that
-- was stored as Unix epoch milliseconds before the timestamp unit was unified.
UPDATE market_data
SET
    date_start = date_start * 1000,
    date_end = date_end * 1000
WHERE date_start < 100000000000000;

UPDATE backtest_data
SET
    date_start = date_start * 1000,
    date_end = date_end * 1000
WHERE date_start < 100000000000000;
