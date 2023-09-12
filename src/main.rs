extern crate csv;

use clap::Parser;
use std::{collections::HashSet, error::Error, fs::File};

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
    about = "Useful tool to search for Pi wallet address based on partial data"
)]
struct Args {
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
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file = File::open("accounts.csv")?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut matched_args = Args::parse();
    matched_args.to_uppercase();

    let mut possible_wallets = HashSet::new();

    // Iterate over each record
    for result in rdr.records() {
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
