use rusqlite::{params, Connection, Result};

/// id -> db index;
/// ytb_id -> video ID on youtube;
/// playlist ID on youtube;
#[allow(dead_code)]
pub struct VideoDB {
    ytb_id: String,
    playlist_id: String,
}
#[allow(dead_code)]
impl VideoDB {
    pub fn new(ytb_id: String, playlist_id: String) -> Self {
        VideoDB { ytb_id, playlist_id }
    }
}

/// connect to SQLite database
#[allow(dead_code)]
pub fn connect() -> Connection {
    
    let conn: Result<Connection> = Connection::open("database\\playlist.db");
    
    match conn {
        Ok(db_con) => {
            log::info!("succesfully connected to db");
            return db_con
        },
        Err(err) => {
            log::error!("failed to connect to db : {}", err);
            std::process::exit(1);
        }
    }
}

/// check if a table exists
#[allow(dead_code)]
pub fn table_exists(conn: &Connection, table_name: &String) -> bool {

    let query = format!("SELECT name FROM sqlite_master WHERE type='table' AND name='{}';", table_name);

    match conn.prepare(&query) {
        Ok(mut stmt) => {
            let exists = stmt.exists([]);
            return exists.unwrap();
        }
        Err(err) => {
            log::error!("failed to check if table exists : {}", err);
            std::process::exit(1);
        }
    }
}

/// create a new table for each new playlist URL
#[allow(dead_code)]
pub fn create_table(conn: &Connection, table_name: &String) {

    let query = format!("CREATE TABLE IF NOT EXISTS {} (ytbid text NOT NULL);", table_name);

    match conn.execute(
        &query,
        params![],
    )
    {
        Ok(_) => log::info!("new playlist saved succesfully in the database"),
        Err(err) => handle_sql_error(format!("failed to create table : {}", err)),
    }
}

/// insert a new video into the database
#[allow(dead_code)]
pub fn insert_new_video(conn: &Connection, video: VideoDB) {

    match conn.execute(
        &format!("INSERT INTO {} (ytbid) VALUES (?);", video.playlist_id),
        params![video.ytb_id],
    )
    {
        Ok(_) => (),
        Err(err) => handle_sql_error(format!("inserting new video : {}", err)),
    }
}

/// get all video ID from a playlist (table name)
#[allow(dead_code)]
pub fn get_video_from_db(conn: &Connection, playlist_id: &String) -> Result<Vec<String>> {


    let sql = format!("SELECT ytbid FROM {}", playlist_id);

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], |row| row.get(0))?;

    let mut ytb_ids = Vec::new();
    for result in rows {
        let ytb_id: String = result?;
        ytb_ids.push(ytb_id);
    }

    Ok(ytb_ids)

}

fn handle_sql_error(msg: String) {
    log::error!("{}", msg);
    std::process::exit(1);
} 

// SQL SCHEMA

// one TABLE by playlist

// id integer PRIMARY KEY
// ytbid text NOT NULL

// https://www.coderstool.com/sql-syntax-checker