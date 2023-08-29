use gpt_http::gpt_handlers::make_prompt;
use string_builder::Builder;
use colored::Colorize;
use rustbreak::{FileDatabase};


mod gpt_http {
    pub mod gpt_handlers;
    pub mod ext_consts;
}
 

fn main() {
    let mut prompt: String = String::new();
    let mut builder = Builder::default();
    builder.append("\n\nHello, welcome to cli chart assistant, these are your commands, \n".blue().to_string());
    builder.append("1. ".red().to_string());
    builder.append( "for chat\n".blue().to_string());
    builder.append( "2. ".red().to_string());
    builder.append( "to see history chat history and \n".blue().to_string());
    builder.append( "0. ".red().to_string());
    builder.append( "to exit the app\n\n".blue().to_string());
    
    println!("{}", builder.string().unwrap());
    std::io::stdin().read_line(&mut prompt).unwrap();

    let gpt_res = make_prompt(&prompt);

    match gpt_res {
        Ok(chart_response) => {
            // You successfully got a ChartResponse
            println!("Parsed response: {:?}", chart_response.choices[0].message.content);
        }
        Err(err) => {
            // Handle the error
            eprintln!("Error: {:?}", err);
        }
    }
}
