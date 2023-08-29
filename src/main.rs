use std::process::exit;
use std::io::{ Write };

use gpt_http::gpt_handlers::make_prompt;
use string_builder::Builder;
use colored::Colorize;

use crate::gpt_http::gpt_handlers::ChatResponse;

mod gpt_http {
    pub mod gpt_handlers;
    pub mod ext_consts;
}

// #[tokio::main]
// fn main() {
// let mut option: i8 = -1;
// let mut input_line = String::new();

// let mut builder = Builder::default();
// builder.append(
//     "\n\nHello, welcome to cli chat assistant, these are your commands, \n".blue().to_string()
// );
// builder.append("1. ".red().to_string());
// builder.append("for chat\n".blue().to_string());
// builder.append("2. ".red().to_string());
// builder.append("to see history chat history and \n".blue().to_string());
// builder.append("0. ".red().to_string());
// builder.append("to exit the app\n\n".blue().to_string());

// println!("{}", builder.string().unwrap());

//     while option != 0 {
//         if option != 1 {
//             // if option == 1 meaning we are still charting
//             print!("please enter option: ");
//             std::io::stdout().flush().unwrap();
//         }

//         std::io::stdin().read_line(&mut input_line).unwrap();
//         option = input_line.trim().parse().expect("Input not an integer");
//         match option {
//             1 => {
//                 option = chat(option);

//                 std::io::stdout().flush().unwrap();
//                 println!("{}", "say something >> ".green().to_string());
//             }
//             2 => see_chat_history(),
//             0 => exit_app(),
//             _ => print!("please enter correct option: "),
//         }
//     }
// }

fn main() {
    let mut option: i8 = -1;
    let mut input_line = String::new();

    let mut builder = Builder::default();
    builder.append(
        "\n\nHello, welcome to cli chat assistant, these are your commands, \n".blue().to_string()
    );
    builder.append("1. ".red().to_string());
    builder.append("for chat\n".blue().to_string());
    builder.append("2. ".red().to_string());
    builder.append("to see history chat history and \n".blue().to_string());
    builder.append("0. ".red().to_string());
    builder.append("to exit the app\n\n".blue().to_string());

    println!("{}", builder.string().unwrap());

    while option != 0 {
        if option != 1 {
            print!("please enter option: ");
            std::io::stdout().flush().unwrap();
        }

        std::io::stdin().read_line(&mut input_line).unwrap();
        option = input_line.trim().parse().expect("Input not an integer");

        match option {
            1 => chat(),
            2 => see_chat_history(),
            0 => exit_app(),
            _ => print!("please enter correct option: "),
        }
    }
}

fn see_chat_history() {
    println!("not today")
}

fn exit_app() {
    exit(0)
}

fn chat() {
    let mut prompt: String = String::new();

    while !prompt.eq("-1") {
        prompt.clear();

        println!("{}", "say something >> ".yellow().to_string());
        std::io::stdin().read_line(&mut prompt).unwrap();
        let gpt_res = make_prompt(&prompt);

        match gpt_res {
            Ok(Some(chart_response)) => {
                // You successfully got a ChartResponse
                let mut builder = Builder::default();
                builder.append("GPT response: ".blue().to_string());
                builder.append(
                    (chart_response as ChatResponse).choices[0].message.content.green().to_string()
                );
                println!("{}", builder.string().unwrap());
            }
            Ok(None) => {
                println!("No response");
            }
            Err(err) => {
                // Handle the error
                eprintln!("Error: {:?}", err);
            }
        }
    }
}
