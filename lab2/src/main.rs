// use std::io;
// use actix::prelude::*;
// use indicatif::ProgressBar;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::process;
use dotenv::dotenv;
use structopt::StructOpt;



/*
 * Some aspects of this application were inspired by the awesome guide on
 * Rust CLI applications that can be found here
 * https://rust-lang-nursery.github.io/cli-wg/index.html
 */ 
fn main() {
    /* 
     * Set up the panic handler to print a human-readable error message to the terminal
     * and write the termination backtrace log to a temporary file.
     * This reads cargo metadata such as the application and author name at
     * compile time and displays it when the `panic!()` macro invokation was hit
     * by the workflow.
     */
    human_panic::setup_panic!();

    if !dotenv().is_ok() {
        eprintln!("Failed to load \".env\" file, please put one in your current working directory");
        process::exit(exitcode::CONFIG);
    }

    /*
     * Rust supports return-type polymorphism. The variable type annotation
     * participates in function template deduction.
     */
    let config: lab2::Config = envy::from_env().unwrap_or_else(|err| {
        eprintln!("Failed to read valid configuration from environment variables: {}", err);
        process::exit(exitcode::CONFIG);
    });

    /*
     * This is on of the first statements of the program to ensure that the user
     * did pass some adequate parameters to be processed. Otherise the
     * "clap" CLI framework prints the help/error message and terminates the
     * workflow. This means the following statement never returns if no valid
     * parameters were read from the prompt.
     */  
    let cli_params = lab2::CliParams::from_args();

    let db_conn = PgConnection::establish(&config.database_url).unwrap_or_else(|_| {
        // Don't expose database_url since it may expose user credentials.
        eprintln!(
            "Failed to establish connection with PostgreSQL, please check the \
             validity of DATABASE_URL evironment variable."
        );
        process::exit(exitcode::UNAVAILABLE);
    });

    lab2::run(db_conn, cli_params);
}
