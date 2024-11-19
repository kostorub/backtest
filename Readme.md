# The backtest written in Rust
## Description
The goal of this project is to make a backtesting framework for the most popular trading strategies like grid or rebalancing. Before that, I did the same with Python programming language and even used PyPy (which increased the performance in times), but the performance left much to be desired. So I decided to use the Rust programming language to benefit from performance and code safety.

## Finished parts
- Download and use Binance's historical market data (including 1s & trades)
- Iteration over the same-period chunks of klines to implement complex closely-related strategies with low-memory consumption
- Launch of the Binance Grid bot trading strategy even on trades market data
- Calculation of metrics
- Simple UI built using the HTMX tool with the result chart construction

## Preview
![image](https://github.com/kostorub/backtest/assets/11979976/745dd00f-1c3e-41f2-b75f-c0d7da28301b)

### Binance's historical market data
The historical data is loaded from the [Binance Data Collection](https://data.binance.vision) source. The data is stored as in the following structure in the file system to allow easy access by the [memory mapped IO](https://docs.rs/memmap2/latest/memmap2/).  

|timestamp: 8 bytes|open: 8 bytes|high: 8 bytes|low: 8 bytes|close: 8 bytes|volume: 8 bytes|
|---|---|---|---|---|---|

Klines are going one by one.
|01-01-2000 12:00:00|01-01-2000 12:00:01|01-01-2000 12:00:02|...|N|
|---|---|---|---|---|

The backtesting engine takes the data in the range of `date_start` and `date_end` using the timestamp offset.
To use the offset, gaps in the klines sequence (if they exist) are filled with the previous valid kline data.

### Grid bot strategy
The grid bot strategy was implemented according to the Binance grid bot description. Here are a couple of links to the mentioned description: [What Is Spot Grid Trading and How Does It Work](https://www.binance.com/en/support/faq/what-is-spot-grid-trading-and-how-does-it-work-d5f441e8ab544a5b98241e00efb3a4ab), [How to Create a Spot Grid Trading Strategy on Binance](https://www.binance.com/en/support/faq/how-to-create-a-spot-grid-trading-strategy-on-binance-95078b6293184bd79b56108092f337c1?hl=en) and [Binance Spot Grid Trading Parameters](https://www.binance.com/en/support/faq/binance-spot-grid-trading-parameters-688ff6ff08734848915de76a07b953dd?hl=en).  
There is only one symbol to calculate available, but core functions and the multithreaded background of the framework are intended to backtest several symbols at once to get a big picture simultaneously.

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
## SQLite DB

> The choice to use the SQLite database in this project is driven by cost-efficiency considerations. Since the project does not require the handling of large volumes of data, a lightweight and low-cost solution like SQLite is suitable. Additionally, the project does not face significant demands in terms of concurrent data access during backtests processes, minimizing the need for a more robust database system with higher concurrency capabilities. SQLite also offers a mechanism to optimize concurrency through its [Write-Ahead Logging (WAL)](https://www.sqlite.org/wal.html) feature. This feature allows for improved concurrency by permitting multiple readers to access the database at the same time a write operation is occurring, thereby enhancing performance without compromising data integrity. This makes SQLite an effective choice for projects with moderate concurrency requirements and a need for cost-effective data management solutions.

The database is used to store the following data:
- users
- information about already downloaded historical market data
- the initial parameters and metrics of processed backtests

The database is shared between kubernetes nodes by mounting the [PersistentVolumeClaim](https://kubernetes.io/docs/concepts/storage/persistent-volumes/) to each node. [Digital Ocean: How to Add Volumes to Kubernetes Clusters](https://docs.digitalocean.com/products/kubernetes/how-to/add-volumes/)

# CI/CD
The CI/CD process consists of two steps:
- Building the container with the service and storing it in the docker hub service.
- Running the deployment job to install the image on the Digital Ocean k8s cluster.

Check [the digital-ocean.yml file](https://github.com/kostorub/backtest/blob/main/.github/workflows/digital-ocean.yml) to view all the steps.
## Development notes and auxiliary commands
### Kubernetes
Connect
```
doctl kubernetes cluster kubeconfig save $CLUSTER_ID
```
### Sqlite
Create the local .env file
```bash
cp .env.template .env
```
Set the following variables:
- DATA_PATH (i.e. data)
- DATABASE_PATH (i.e. db)
- DATABASE_URL (i.e. sqlite:db/backtest.sqlite)

Create the DB
```bash
mkdir db
sqlx db create --sqlite-create-db-wal=false
```
Migrate DB
```bash
sqlx migrate run
```
Prepare for the offline check and build
```bash
cargo sqlx prepare
```
Create new migration sql file
```bash
cargo sqlx migrate add -r -s <name>
# cargo sqlx migrate add -r -s key_value_store
```
Then don't forget to set `DATABASE_MIGRATION_VERSION` in .env.template

### How to deploy new version

These commands will trigger the deploy job  
Get the latest tag
```bash
git describe --tags
# v0.7
```
Set the incremented one
```bash
git tag v0.8
```
And push
```bash
git push --tags
```
