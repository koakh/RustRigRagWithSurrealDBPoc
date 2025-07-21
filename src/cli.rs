use clap::{Arg, Command};

pub struct Cli {}

impl Cli {
    pub fn run() {
        let mut cmd = Command::new("RigRagPoc")
            .version("1.0")
            .about("Rig Rag Rust PoC CLI")
            // .subcommand(
            //     Command::new("add")
            //         .about("Adds two numbers")
            //         .arg(
            //             Arg::new("num1")
            //                 .help("First number")
            //                 .required(true)
            //                 .index(1),
            //         )
            //         .arg(
            //             Arg::new("num2")
            //                 .help("Second number")
            //                 .required(true)
            //                 .index(2),
            //         ),
            // )
            // .subcommand(
            //     Command::new("greet")
            //         .about("Greets the user")
            //         .arg(Arg::new("name").help("Your name").required(true).index(1)),
            // )
            .subcommand(
                Command::new("init")
                    .about("Init RAG system: SurrealDB Schema and Sample Documents"),
            )
            .subcommand(Command::new("query").about("Greets the user"));

        let matches = cmd.clone().get_matches();

        match matches.subcommand() {
            // Some(("add", sub_matches)) => {
            //     let num1: i32 = sub_matches
            //         .get_one::<String>("num1")
            //         .unwrap()
            //         .parse()
            //         .unwrap();
            //     let num2: i32 = sub_matches
            //         .get_one::<String>("num2")
            //         .unwrap()
            //         .parse()
            //         .unwrap();
            //     println!("Sum: {}", num1 + num2);
            // }
            // Some(("greet", sub_matches)) => {
            //     if let Some(name) = sub_matches.get_one::<String>("name") {
            //         println!("Hello, {}!", name);
            //     }
            // }
            Some(("init", sub_matches)) => {
                println!("Init");
            }
            Some(("query", sub_matches)) => {
                println!("query");
            }
            _ => {
                // Print help if no subcommand is matched
                let _ = cmd.print_help();
                // Add newline after help
                println!();
            }
        }
    }
}
