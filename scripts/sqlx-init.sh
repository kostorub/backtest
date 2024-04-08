#!/bin/sh
sqlx db create --sqlite-create-db-wal=false
sqlx migrate run
