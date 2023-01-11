use clap::{command, Command};
use rusqlite::Connection;
use todolist::run;

fn main() {
    let matches = command!()
        .subcommand(Command::new("add").about("Add a todo"))
        .subcommand(Command::new("remove").about("remove a todo"))
        .subcommand(Command::new("update").about("update a todo"))
        .subcommand(Command::new("list").about("list a todo"))
        .get_matches();
    let connection = connect_to_database();
    run(matches, connection);
}

fn connect_to_database() -> Connection {
    let connection = Connection::open("./todos.db").unwrap();
    let sql = "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL UNIQUE
    )";
    connection.execute(sql, []).unwrap();
    connection
}
