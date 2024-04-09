use std::str::FromStr;

use sqlx::Error;
use sqlx::SqlitePool;

use crate::{
    data_handlers::utils::u64_to_datetime_str,
    data_models::market_data::{
        enums::MarketDataType,
        market_data::{GetMarketDataRequest, MarketDataFront},
    },
};

pub async fn insert_market_data(
    pool: &SqlitePool,
    exchange: String,
    symbol: String,
    market_data_type: MarketDataType,
    date_start: u64,
    date_end: u64,
) -> Result<(), Error> {
    let market_data = get_market_data_one(pool, &exchange, &symbol, &market_data_type).await?;

    let market_data_type = market_data_type.value().0;
    let date_start = date_start as i64;
    let date_end = date_end as i64;

    match market_data {
        Some(data) => {
            let id = data.id.unwrap();
            sqlx::query!(
                "UPDATE market_data SET date_start = ?1, date_end = ?2 WHERE id = ?3",
                date_start,
                date_end,
                id
            )
            .execute(pool)
            .await?;
        }
        None => {
            sqlx::query!(
                "INSERT INTO market_data (exchange, symbol, market_data_type, date_start, date_end)
                VALUES (?1, ?2, ?3, ?4, ?5)",
                exchange,
                symbol,
                market_data_type,
                date_start,
                date_end
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

pub async fn get_market_data_one(
    pool: &SqlitePool,
    exchange: &String,
    symbol: &String,
    market_data_type: &MarketDataType,
) -> Result<Option<MarketDataFront>, Error> {
    let market_data_type = market_data_type.value().0;
    let row = sqlx::query!(
        "SELECT * FROM market_data WHERE exchange = ?1 AND symbol = ?2 AND market_data_type = ?3",
        exchange,
        symbol,
        market_data_type
    )
    .fetch_optional(pool)
    .await?
    .map(|row| MarketDataFront {
        id: Some(row.id),
        exchange: row.exchange,
        symbol: row.symbol,
        market_data_type: MarketDataType::from_str(String::as_str(&row.market_data_type)).unwrap(),
        date_start: u64_to_datetime_str(row.date_start as u64),
        date_end: u64_to_datetime_str(row.date_end as u64),
    });

    Ok(row)
}

pub async fn get_market_data_page(
    pool: &SqlitePool,
    r: &GetMarketDataRequest,
) -> Result<Vec<MarketDataFront>, Error> {
    let offset = r.page * r.per_page;
    let rows: Vec<MarketDataFront> = sqlx::query!(
        "SELECT * FROM market_data LIMIT ?1 OFFSET ?2",
        r.per_page,
        offset
    )
    .fetch_all(pool)
    .await?
    .iter()
    .map(|row| MarketDataFront {
        id: Some(row.id),
        exchange: row.exchange.clone(),
        symbol: row.symbol.clone(),
        market_data_type: MarketDataType::from_str(String::as_str(&row.market_data_type)).unwrap(),
        date_start: u64_to_datetime_str(row.date_start as u64),
        date_end: u64_to_datetime_str(row.date_end as u64),
    })
    .collect();

    Ok(rows)
}
