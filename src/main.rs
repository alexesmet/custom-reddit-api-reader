mod lib;

use lib::configuration::get_configuration;

#[tokio::main]
async fn main() {

    let confuguration = get_configuration().expect("Configuration file should have correct format");

    if let Err(err) = lib::run(confuguration).await {
        match err {
            lib::AppError::AuthRequestFailed (e) => eprintln!("Authentication request failed: {e}"),
            lib::AppError::AuthResponseUnreadable (e) => eprintln!("Failed to read authentication response: {e}"),
            lib::AppError::ReadRequestFailed (e) => eprintln!("Request to read reddit data failed: {e}"),
            lib::AppError::ReadResponseUnreadable (e) => eprintln!("Can't read reddit data response: {e}"),
            lib::AppError::OutputFailed (e) => eprintln!("There was a failure to output the data: {e}"),
            lib::AppError::IO (e) => eprintln!("Problem writing to file: {e}")
        }
        std::process::exit(1);
    }

}
