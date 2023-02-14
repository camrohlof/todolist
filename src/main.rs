use rusqlite::Connection;
use todolist::{run, Cli};
fn main() {
    let cli = <Cli as clap::Parser>::parse();
    let connection = connect_to_database();
    run(cli, connection);
}

fn connect_to_database() -> Connection {
    let connection = Connection::open("./todos.sqlite").unwrap();
    let sql = "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        details TEXT,
        completed BOOLEAN DEFAULT false
    )";
    connection.execute(sql, []).unwrap();
    connection
}
