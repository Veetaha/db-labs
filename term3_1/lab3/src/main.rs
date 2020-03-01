use std::{process, time::Duration};
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
    if !cfg!(debug_assertions) { // conditional compile this for production only
        human_panic::setup_panic!();
    }

    if !dotenv().is_ok() {
        eprintln!("Failed to load \".env\" file, please put one in your current working directory");
        process::exit(exitcode::CONFIG);
    }

    /*
     * Rust supports return-type polymorphism. The variable type annotation
     * participates in function template deduction.
     */
    let config: lab3::Config = envy::from_env().unwrap_or_else(|err| {
        eprintln!("Failed to read valid configuration from environment variables: {}", err);
        process::exit(exitcode::CONFIG);
    });

    /*
     * This is one of the first statements of the program to ensure that the user
     * did pass some adequate parameters to be processed. Otherise the
     * "clap" CLI framework prints the help/error message and terminates the
     * workflow. This means the following statement never returns if no valid
     * parameters were read from the prompt.
     */  
    let cli_params = lab3::cli::Params::from_args();

    let timeout = Duration::from_millis(config.db_connect_timeout);

    use lab3::PgConnManager;

    let pool = lab3::PgConnPool::builder()
        .connection_timeout(timeout)
        .max_size(1)
        .build(PgConnManager::new(config.database_url))
        .unwrap_or_else(|err| {
            eprintln!(
                "Failed to establish connection to database, please check the \
                correctness of configuration evironment variables or your \
                nerwork connection: {}",
                err
            );
            process::exit(exitcode::UNAVAILABLE);
        });

    if let Err(err) = lab3::run(pool, cli_params) {
        eprintln!("Error: {}", err);
        process::exit(exitcode::SOFTWARE);
    }
}
