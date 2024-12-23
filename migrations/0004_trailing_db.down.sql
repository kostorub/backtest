-- Add down migration script here
ALTER TABLE grid_data RENAME TO backtest_data;
DROP TABLE trailing_data;