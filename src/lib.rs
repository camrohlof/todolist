use std::{fmt, process};

use clap::{Parser, Subcommand};
use rusqlite::Connection;
use text_io::read;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        name: String,
        details: Option<String>,
    },
    Remove {
        name: String,
    },
    Describe {
        name: String,
        details: String,
    },
    Finish {
        name: String,
    },
}

#[derive(Debug)]
struct TodoItem {
    name: String,
    details: String,
    completed: bool,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} | Finished[{}]",
            self.name, self.details, self.completed
        )
    }
}

pub fn run(cli: Cli, connection: Connection) {
    match cli.command {
        Some(commands) => match commands {
            Commands::Add { name, details } => {
                add_item(connection, name, details);
            }
            Commands::Remove { name } => remove_item(connection, name),
            Commands::Describe { name, details } => update_item(connection, name, details),
            Commands::Finish { name } => {
                complete_item(connection, name);
            }
        },
        None => list_items(connection),
    };
}

fn add_item(connection: Connection, name: String, details: Option<String>) {
    let mut default = "No details provided.".to_owned();
    if let Some(details) = details {
        default = details;
    }

    let result = connection.execute(
        "INSERT INTO todo (name, details) values (?1,?2)",
        [&*name, &*default],
    );
    match result {
        Ok(_) => println!("Created todo named: {name}"),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1)
        }
    }
}

fn remove_item(connection: Connection, name: String) {
    let mut statement = connection
        .prepare("SELECT * FROM todo WHERE name=?")
        .unwrap();
    let item = statement
        .query_row([&name], |row| {
            Ok(TodoItem {
                name: row.get(1)?,
                details: row.get(2)?,
                completed: row.get(3)?,
            })
        })
        .unwrap();

    println!(
        "Are you sure you would like to remove: [{}]? (Y,n)",
        item.name.trim_end(),
    );
    let response: String = read!();
    if response.to_lowercase() == "n" || response.to_lowercase() == "no" {
        println!("Exiting.");
        process::exit(1)
    } else {
        let mut statement = connection.prepare("DELETE FROM todo WHERE name=?").unwrap();
        let result = statement.execute([item.name]).unwrap();
    }
}

fn update_item(connection: Connection, name: String, details: String) {
    let mut statement = connection
        .prepare("SELECT * FROM todo WHERE name=?")
        .unwrap();
    let item = statement
        .query_row([&name], |row| {
            Ok(TodoItem {
                name: row.get(1)?,
                details: row.get(2)?,
                completed: row.get(3)?,
            })
        })
        .unwrap();

    println!("Change the details of: [{}]? (Y,n)", item.name.trim_end(),);
    let response: String = read!();
    if response.to_lowercase() == "n" || response.to_lowercase() == "no" {
        println!("Exiting.");
        process::exit(1)
    } else {
        let mut statement = connection
            .prepare("UPDATE todo SET details=? WHERE name=?")
            .unwrap();
        let result = statement.execute([item.details, item.name]).unwrap();
    }
}

fn list_items(connection: Connection) {
    let mut statement = connection
        .prepare("SELECT * FROM todo WHERE completed=False")
        .unwrap();
    let results = statement
        .query_map([], |row| {
            Ok(TodoItem {
                name: row.get(1)?,
                details: row.get(2)?,
                completed: row.get(3)?,
            })
        })
        .unwrap();
    for result in results {
        match result {
            Ok(result) => println!("{result}"),
            Err(error) => eprintln!("{error}"),
        }
    }
}

fn complete_item(connection: Connection, name: String) {
    let mut statement = connection
        .prepare("UPDATE todo SET completed=True WHERE name=?")
        .unwrap();
    let result = statement.execute([name]).unwrap();
}
