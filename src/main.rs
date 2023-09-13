extern crate csv;

use clap::Parser;
use std::{collections::HashSet, error::Error, io, io::BufRead, fs::{metadata,File}};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Record {
    block_number: u64,
    total_amount: u64,
    wallet_address: String,
    timestamp: String,
}

#[derive(Debug, Clone, Parser)]
#[clap(
    version,
    about = "Useful tool to search for Pi wallet address or transaction hash based on partial data"
)]
struct Args {
    #[clap(short = 'm', long = "mode", default_value = "wallet", help = "Depends you are looking for wallet then give wallet or account, if you looking for transaction hash give hash or txhash value")]
    mode: String,
    #[clap(
        short,
        long = "begins",
        help = "The exact characters beginning of the wallet, starting with G, please use no more than 28!"
    )]
    begins: Option<String>,
    #[clap(
        short,
        long = "ends",
        help = "The exact characters ending of the wallet of the wallet please use no more than 28!"
    )]
    ends: Option<String>,
    #[clap(
        short,
        long = "contains",
        help = "Characters in the exact order which are in the wallet! Highly optional."
    )]
    contains: Option<String>,
}

impl Args {
    // Function to convert all arguments to uppercase because the user could use lowercase input accidentally
    fn to_uppercase(&mut self) {
        let _ = &mut self.mode.make_ascii_uppercase();
        if let Some(ref mut begins) = &mut self.begins {
            begins.make_ascii_uppercase();
        }
        if let Some(ref mut ends) = &mut self.ends {
            ends.make_ascii_uppercase();
        }
        if let Some(ref mut contains) = &mut self.contains {
            contains.make_ascii_uppercase();
        }
    }

    fn is_empty(&self) -> bool {
        let statement = self.begins.is_some() || self.ends.is_some() || self.contains.is_some();
        !statement
    }

    fn validate_mode(&self) -> bool {
        self.mode == "WALLET" || self.mode == "ACCOUNT" || self.mode == "HASH" || self.mode == "TXHASH"
    }
}

fn solve_account(acc_rdr_orginal: Option<csv::Reader<File>>, matched_args: Args) -> Result<(), Box<dyn Error>> {
    let mut possible_wallets = HashSet::new();
    let mut acc_rdr = acc_rdr_orginal.expect("The accounts.csv not loaded correctly!\nAbort");

    // Iterate over each record
    for result in acc_rdr.records() {
        let record = result?;
        let block_number: u64 = record[0].parse()?;
        let total_amount: u64 = record[1].parse()?;
        let wallet_address = record[2].to_string();
        let timestamp = record[3].to_string();

        let entry = Record {
            block_number,
            total_amount,
            wallet_address,
            timestamp,
        };

        // Case when we get all the values
        if let (Some(begins), Some(ends), Some(contains)) = (
            &matched_args.begins,
            &matched_args.ends,
            &matched_args.contains,
        ) {
            if entry.wallet_address.starts_with(begins)
                && entry.wallet_address.ends_with(ends)
                && entry.wallet_address.contains(contains)
            {
                possible_wallets.insert(entry.clone());
            }
        }
        // If the begins and ends are known only
        else if let (Some(begins), Some(ends)) = (&matched_args.begins, &matched_args.ends) {
            if entry.wallet_address.starts_with(begins) && entry.wallet_address.ends_with(ends) {
                possible_wallets.insert(entry.clone());
            }
        }
        // If the begins and contains known only
        else if let (Some(begins), Some(contains)) =
            (&matched_args.begins, &matched_args.contains)
        {
            if entry.wallet_address.starts_with(begins) && entry.wallet_address.contains(contains) {
                possible_wallets.insert(entry.clone());
            }
        }
        // If the ends and contains known only
        else if let (Some(ends), Some(contains)) = (&matched_args.ends, &matched_args.contains) {
            if entry.wallet_address.ends_with(ends) && entry.wallet_address.contains(contains) {
                possible_wallets.insert(entry.clone());
            }
        }
        // If only the begins known
        else if let Some(begins) = &matched_args.begins {
            if entry.wallet_address.starts_with(begins) {
                possible_wallets.insert(entry.clone());
            }
        }
        // If only the ends known
        else if let Some(ends) = &matched_args.ends {
            if entry.wallet_address.ends_with(ends) {
                possible_wallets.insert(entry.clone());
            }
        }
        // If only the contains known
        else if let Some(contains) = &matched_args.contains {
            if entry.wallet_address.contains(contains) {
                possible_wallets.insert(entry.clone());
            }
        } else {
            println!("You must give some argument! Read --help for more information.");
            break;
        }
    }

    if possible_wallets.is_empty() && !matched_args.is_empty() {
        println!("No wallets found with the specified criteria.");
    } else if !matched_args.is_empty() {
        println!("Possible wallets:");
        for wallet in possible_wallets {
            println!("Address could be: {} \nIt was created in block {} and it was the {}th wallet created. \nDate of creation: {} \n----------------------------------------------------------------------------", wallet.wallet_address, wallet.block_number, wallet.total_amount, wallet.timestamp);
        }
    }
    Ok(())
}

fn solve_hash(hash_rdr_orginal: Option<io::BufReader<File>>, matched_args: Args) -> Result<(), Box<dyn Error>>  {
    let mut possible_hash = HashSet::new();
    let hash_rdr = hash_rdr_orginal.expect("The hashlist.txt not loaded correctly!\nAbort");

    // Iterate over each line in the text file
    for line in hash_rdr.lines() {
        let line = line?;
        let entry = line.trim().to_uppercase();

        // Case when we get all the values
        if let (Some(begins), Some(ends), Some(contains)) = (
            &matched_args.begins,
            &matched_args.ends,
            &matched_args.contains,
        ) {
            if entry.starts_with(begins)
                && entry.ends_with(ends)
                && entry.contains(contains)
            {
                possible_hash.insert(entry.clone());
            }
        }
        // If the begins and ends are known only
        else if let (Some(begins), Some(ends)) = (&matched_args.begins, &matched_args.ends) {
            if entry.starts_with(begins) && entry.ends_with(ends) {
                possible_hash.insert(entry.clone());
            }
        }
        // If the begins and contains known only
        else if let (Some(begins), Some(contains)) =
            (&matched_args.begins, &matched_args.contains)
        {
            if entry.starts_with(begins) && entry.contains(contains) {
                possible_hash.insert(entry.clone());
            }
        }
        // If the ends and contains known only
        else if let (Some(ends), Some(contains)) = (&matched_args.ends, &matched_args.contains) {
            if entry.ends_with(ends) && entry.contains(contains) {
                possible_hash.insert(entry.clone());
            }
        }
        // If only the begins known
        else if let Some(begins) = &matched_args.begins {
            if entry.starts_with(begins) {
                possible_hash.insert(entry.clone());
            }
        }
        // If only the ends known
        else if let Some(ends) = &matched_args.ends {
            if entry.ends_with(ends) {
                possible_hash.insert(entry.clone());
            }
        }
        // If only the contains known
        else if let Some(contains) = &matched_args.contains {
            if entry.contains(contains) {
                possible_hash.insert(entry.clone());
            }
        } else {
            println!("You must give some argument! Read --help for more information.");
            break;
        }
    }

    if possible_hash.is_empty() && !matched_args.is_empty() {
        println!("No hash found with the specified criteria.");
    } else if !matched_args.is_empty() {
        println!("Possible hash:");
        for hash in possible_hash {
            println!("Hash: {} \n-------------------------------------------------------------------------", hash.to_lowercase());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Conditional loading of the CSV file
    let mut acc_rdr: Option<csv::Reader<File>> = None;
    let accounts_file_path = "accounts.csv";
    
    if let Ok(metadata) = metadata(accounts_file_path) {
        if metadata.is_file() {
            // The file exists, let's try to open it
            let csv_file = File::open(accounts_file_path)?;
            acc_rdr = Some(csv::ReaderBuilder::new()
                .has_headers(true)
                .from_reader(csv_file));
        } else {
            println!("The specified path is not a file.");
            return Ok(());
        }
    } else if let Err(error) = metadata(accounts_file_path) {
        if error.kind() == io::ErrorKind::NotFound {
            println!("The accounts.csv file does not exist. If you want to check wallets also get it!");
        } else {
            println!("Error checking file existence: {:?}", error);
            return Err(Box::new(error));
        }
    }
    
    // Conditional loading of the TXT file
    let hash_file_path = "hashlist.txt";
    let mut hash_rdr: Option<io::BufReader<File>> = None;

    if let Ok(metadata) = metadata(hash_file_path) {
        if metadata.is_file() {
            // The file exists, let's try to open it
            let txt_file = File::open(hash_file_path)?;
            hash_rdr = Some(io::BufReader::new(txt_file));
        } else {
            println!("The specified path is not a file.");
            return Ok(());
        }
    } else if let Err(error) = metadata(hash_file_path) {
        if error.kind() == io::ErrorKind::NotFound {
            println!("The hashlist.txt file does not exist. If you want to check hashes also get it!");
        } else {
            println!("Error checking file existence: {:?}", error);
            return Err(Box::new(error));
        }
    }

    let mut matched_args = Args::parse();
    matched_args.to_uppercase();

    if !matched_args.validate_mode() {
        println!("You must give valid mode. Read --help for more information which modes are working");
        return Ok(());
    }

    match matched_args.mode.as_str() {
        "WALLET" | "ACCOUNT" => {let _ = solve_account(acc_rdr, matched_args);},
        "HASH" | "TXHASH" => {let _ = solve_hash(hash_rdr, matched_args);},
        _ => {println!("How you did this?");},
    } 

    Ok(())
}
