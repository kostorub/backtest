#!/bin/bash
set -x

/opt/app/sqlx database drop -y
/opt/app/sqlx db create --sqlite-create-db-wal=false
/opt/app/sqlx migrate run
