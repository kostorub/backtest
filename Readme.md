# The backtest written in Rust
## Description
The goal of this project is to make a backtesting framework for the most popular trading strategies like grid or rebalancing. Before that, I did the same with Python programming language and even used PyPy (which increased the performance in times), but the performance left much to be desired. So I decided to use the Rust programming language to benefit from performance and code safety.

## Finished parts
- Download and use Binance's historical market data
- Launch of a Grid bot trading strategy
- Calculation of metrics
- Simple user interface in the HTMX format

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
