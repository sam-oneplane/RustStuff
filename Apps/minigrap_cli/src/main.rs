use std::process;
use std::env;
use minigrap_cli::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();
    // unwrap_or_else receives closer(lambda in C++) 
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);  // print to error stream
        process::exit(1);
    });
    println!("searching from {} in file {}", config.query, config.filename);
    match minigrap_cli::run(config){
        Ok(()) => println!("OK."),
        Err(e) => { 
            eprintln!("App error {}", e);
            process::exit(1);
        }, 
    };
}




