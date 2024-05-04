# The backtest written in Rust
## Description
The goal of this project is to make a backtesting framework for the most popular trading strategies like grid or rebalancing. Before that, I did the same with Python programming language and even used PyPy (which increased the performance in times), but the performance left much to be desired. So I decided to use the Rust programming language to benefit from performance and code safety.

## Finished parts
- Download and use Binance's historical market data
- Launch of a Grid bot trading strategy
- Calculation of metrics
- Simple user interface in the HTMX format

### Binance's historical market data
The historical data is loaded from the [Binance Data Collection](https://data.binance.vision) source. The data is stored as in the following structure in the file system to allow easy access by the [memory mapped IO](https://docs.rs/memmap2/latest/memmap2/).  

|timestamp: 8 bytes|open: 8 bytes|high: 8 bytes|low: 8 bytes|close: 8 bytes|volume: 8 bytes|
|---|---|---|---|---|---|

Klines are going one by one.
|01-01-2000 12:00:00|01-01-2000 12:00:01|01-01-2000 12:00:02|...|N|
|---|---|---|---|---|

The backtesting engine takes the data in the range of `date_start` and `date_end` using the timestamp offset.
To use the offset, gaps in the klines sequence (if they exist) are filled with the previous valid kline data.

## Code structure
<pre>
.
├── .sqlx - folder created by `cargo sqlx prepare` command, it's needed to compile sqlx queries in a GitHub runner
├── migrations - sqlite migrations
└── src
    ├── app_state.rs - actix_web server constant data
    ├── backtest - folder for backtest
    ├── chart - methods for the HTML chart generation
    ├── config.rs - configuration of the service
    ├── data_handlers - handling utilities for the market data mostly
    ├── data_models - data structures and implementations
    ├── database.rs - DB initialization and migration
    ├── db_handlers - sqlx operations
    ├── main.rs - the entry point of the service
    ├── routes - HTTP routes
    ├── server.rs - the server initialization and execution
    ├── tests - tests
    └── web - the HTMX data
</pre>
# CI/CD
## Kubernetes
Connect
```
doctl kubernetes cluster kubeconfig save $CLUSTER_ID
```
## Sqlite
Create the DB
```
mkdir db
sqlx db create --sqlite-create-db-wal=false
```
Migrate DB
```
sqlx migrate run
```
Prepare for the offline check and build
```
cargo sqlx prepare
```
Create new migration sql file
```
cargo sqlx migrate add -r -s <name>
```
Then don't forget to set `DATABASE_MIGRATION_VERSION` in .env.template
