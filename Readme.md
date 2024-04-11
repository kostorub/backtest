# The backtest written in Rust

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
