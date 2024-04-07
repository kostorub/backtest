pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
// pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub async fn init_db(pool: &Pool) {
    let conn = pool.get().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO users (username, password) VALUES ('admin', '89ba60446ddfb9f296863aaa0d6431305fa87c78058d674466672f780be9ecef')",
        [],
    ).unwrap();
}
