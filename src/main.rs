#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use clap::{crate_description, crate_authors, crate_version,
    crate_name, App, AppSettings, Arg, SubCommand};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use failure::Error;
use crate::models::{NewUser, User};

pub mod models;
pub mod schema;

const CMD_ADD: &str = "add";
const CMD_LIST: &str = "list";
const CMD_UPDATE: &str = "update";
const CMD_DELETE: &str = "delete";

fn main() {
    env_loger::init();

    //Parse command line arguments
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("db")
                .value_name("FILE")
                .help("Sets the database file name")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name(CMD_ADD)
                .about("Add a user to database")
                .arg(
                    Arg::with_name("USERNAME")
                        .help("Username of the user to add")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::with_name("EMAIL")
                        .help("E-Mail of the user to add")
                        .required(true)
                        .index(2)
                )
        )
        .subcommand(
            SubCommand::with_name(CMD_LIST)
                .about("Lists the registered users")
        )
        .subcommand(
            SubCommand::with_name(CMD_UPDATE)
                .about("Updates a user")
                .arg(
                    Arg::with_name("USERNAME")
                        .help("Username of the user to be updated")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::with_name("EMAIL")
                        .help("New e-mail of the user")
                        .required(true)
                        .index(2)
                )
        )
        .subcommand(
            SubCommand::with_name(CMD_DELETE)
                .about("Deletes a user")
                .arg(
                    Arg::with_name("USERNAME")
                        .help("Username of the user to delete")
                        .required(true)
                        .index(1)
                )
        )
        .get_matches();

    let path = matches.value_of("database").unwrap_or("test.db");
    let manager = ConnectionManager::<SqliteConnection>::new(path);
    let pool = r2d2::Pool::new(manager).unwrap();

    match matches.subcommand() {
        (CMD_ADD, Some(matches)) => {
            let connection = pool.get().unwrap();
            let username = matches.value_of("USERNAME").unwrap();
            let email = matches.value_of("EMAIL").unwrap();
            let uuid = format!("{}", uuid::Uuid::new_v4());

            let new_user = NewUser {
                id: &uuid,
                username: &username,
                email: &email
            };
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(&connection).unwrap();
        },

        (CMD_LIST, Some(matches)) => {
            use self::schema::users::dsl::*;
            let connection = pool.get().unwrap();

            let mut items = users.load::<models::User>(&connection).unwrap();
            for user in items {
                println!("{:?}", user);
            }
        },

        (CMD_UPDATE, Some(matches)) => {
            use self::schema::users::dsl::*;
            let connection = pool.get().unwrap();
            let u = matches.value_of("USERNAME").unwrap();
            let e = matches.value_of("EMAIL").unwrap();

            diesel::update(users.filter(username.eq(&u)))
                .set(email.eq(&e))
                .execute(&connection);
        },

        (CMD_DELETE, Some(matches)) => {
            use self::schema::users::dsl::*;
            let connection = pool.get().unwrap();
            let u = matches.value_of("USERNAME").unwrap();

            diesel::delete(users.filter(username.eq(&u)))
                .execute(&connection);
        }
        _ => { matches.usage(); }
    }
}
