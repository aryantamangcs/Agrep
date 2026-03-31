use std::env;
use std::error::Error;
use agrep::search;


#[derive(Debug)]
struct Config {
    search_pattern : String,
    file_name : String,
}

impl Config {
    fn build(args : &Vec<String>)-> Result<Config,Box<dyn Error>>{
        if args.len() != 3{
            return Err("Invalid Syntax [cargo run <search_pattern> <file_name>]".into());
        }

        let config = Config{
        search_pattern : args[1].clone(),
        file_name : args[2].clone()
        };
        Ok(config)
    }

}

fn main() {
    let args : Vec<String> = env::args().collect();
    let config = Config::build(&args).expect("[Error]");

    let result = search(&config.search_pattern,&config.file_name).expect("[Error]");
    for r in result.iter(){
        println!("{}",r);
    }

}
