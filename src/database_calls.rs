use rusqlite::{params, Connection, Result};

pub fn create_cardano_price_table() -> Result<()> {
    let conn = Connection::open("sqlite:cryptogains.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cardano_price (
            id INTEGER PRIMARY KEY,
            price REAL NOT NULL,
            date INT NOT NULL
        )",
        params![],
    )?;

    Ok(())
}

pub fn check_last_price_date() -> Result<i64> {
    let conn = Connection::open("sqlite:cryptogains.db")?;

    let mut stmt = conn.prepare("SELECT date FROM cardano_price ORDER BY date DESC LIMIT 1")?;
    let last_date: i64 = stmt.query_row(params![], |row| row.get(0)).unwrap_or(1640930400);
    //the above default is 2021-12-31 00:00:00 UTC
    print!("Last date: {:#?}", &last_date);
    Ok(last_date)
}

pub fn add_price_to_table(price: f64, date: i64) -> Result<()> {
    let conn = Connection::open("sqlite:cryptogains.db")?;

    conn.execute(
        "INSERT INTO cardano_price (price, date) VALUES (?1, ?2)",
        params![price, date],
    )?;
    
    Ok(())
}
