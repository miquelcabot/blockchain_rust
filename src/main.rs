use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(name = "createblockchain", about = "Create a new blockchain")]
    Createblockchain {
        #[arg(help = "The address to send genesis block reward to")]
        address: String,
    },
    #[command(name = "createwallet", about = "Create a new wallet")]
    Createwallet,
    #[command(
        name = "getbalance",
        about = "Get the wallet balance of the target address"
    )]
    GetBalance {
        #[arg(help = "The wallet address")]
        address: String,
    },
    #[command(name = "listaddresses", about = "Print local wallet addresses")]
    ListAddresses,
    #[command(name = "send", about = "Add new block to chain")]
    Send {
        #[arg(help = "Source wallet address")]
        from: String,
        #[arg(help = "Destination wallet address")]
        to: String,
        #[arg(help = "Amount to send")]
        amount: i32,
        #[arg(help = "Mine immediately on the same node")]
        mine: usize,
    },
    #[command(name = "printchain", about = "Print all blocks in the blockchain")]
    Printchain,
    #[command(name = "reindexutxo", about = "Rebuild UTXO index set")]
    Reindexutxo,
    #[command(name = "startnode", about = "Start a node")]
    StartNode {
        #[arg(help = "Enable mining mode and send reward to ADDRESS")]
        miner: Option<String>,
    },
}

fn main() {
    let args: Args = Args::parse();

    match args.command {
        Command::Createblockchain { address } => {
            println!("Create a new blockchain with address: {}", address);
        }
        Command::Createwallet => {
            println!("Create a new wallet");
        }
        Command::GetBalance { address } => {
            println!("Get the wallet balance of the target address: {}", address);
        }
        Command::ListAddresses => {
            println!("Print local wallet addresses");
        }
        Command::Send {
            from,
            to,
            amount,
            mine,
        } => {
            println!(
                "Send {} coins from {} to {} and mine: {}",
                amount, from, to, mine
            );
        }
        Command::Printchain => {
            println!("Print all blocks in the blockchain");
        }
        Command::Reindexutxo => {
            println!("Rebuild UTXO index set");
        }
        Command::StartNode { miner } => {
            println!("Start a node with miner: {:?}", miner);
        }
    }

    /*
    let block = Block::new_block("".to_string(), &[], 0);
    println!("{:?}", block);
    */
}
