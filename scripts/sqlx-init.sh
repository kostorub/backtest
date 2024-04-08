#!/bin/sh
/opt/app/sqlx db create --sqlite-create-db-wal=false
/opt/app/sqlx migrate run
