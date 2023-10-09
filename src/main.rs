use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Check {
        #[arg(short, long)]
        category: String
    },
}




fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::Check { category }) => {
            println!("{}", category);
        },
        _ => {}
    }
}
