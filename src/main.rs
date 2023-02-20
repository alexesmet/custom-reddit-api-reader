mod lib;

use lib::configuration::get_configuration;

#[tokio::main]
async fn main() {
    let confuguration = get_configuration().expect("Configuration file should have correct format");

    lib::run(confuguration).await.unwrap();
    


    

}
