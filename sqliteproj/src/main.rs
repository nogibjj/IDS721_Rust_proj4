/*
A Command-line tool to analyze lyrics to songs and put them into a sqlite database.
 */
 use clap::Parser;

 #[derive(Parser)]
 #[clap(
     version = "1.0",
     author = "Yuxin Song",
     about = "A Command-line tool to analyze with sqlite database."
 )]
 struct Cli {
     #[clap(subcommand)]
     command: Option<Commands>,
 }
 
 #[derive(Parser)]
 enum Commands {
     #[clap(version = "1.0", author = "Yuxin Song")]
     Candidates {
            #[clap(short, long)]
            type: Option<String>,
     },
 }
 
 fn main() {
     let args = Cli::parse();
     match args.command {
         // use get_all_zeroshotcandidates() from lib.rs to get all candidates
         Some(Commands::Candidates {type}) => {
            match type.unwrap().as_str(){
                "rock" => {
                    for candidate in sqliteproj::get_rock_zeroshotcandidates() {
                        println!("{}", candidate);
                    }
                }
                "pop" => {
                    for candidate in sqliteproj::get_pop_zeroshotcandidates() {
                        println!("{}", candidate);
                    }
                }
                "latin" => {
                    for candidate in sqliteproj::get_latin_zeroshotcandidates() {
                        println!("{}", candidate);
                    }
                }
                _ => {
                    for candidate in sqliteproj::get_all_zeroshotcandidates() {
                        println!("{}", candidate);
                    }
                }
            }
            
         }
         None => {
             println!("No command given");
         }
     }
 }