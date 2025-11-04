use solana_client::rpc_client::{self, RpcClient};
use solana_sdk::system_instruction;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;

use std::io;
use std::str::FromStr;

fn get_balance(rpc: &RpcClient, usr_pubkey: Pubkey) -> f64 {
    match rpc.get_account(&usr_pubkey) {
        Ok(account) => {
            let sol_balance: f64 = account.lamports as f64 / LAMPORTS_PER_SOL as f64;
            sol_balance
        },
        Err(_) => 0.0
    }
}

fn send_tx(rpc: &RpcClient, usr: &Keypair) {
    let usr_pubkey = Signer::pubkey(&usr);
    println!("give the address");
    let mut address = String::new();
    io::stdin().read_line(&mut address).expect("error reading input");
    let address_clean = address.trim();
    let to_address = Pubkey::from_str(&address_clean.to_string()).unwrap();

    println!("amount (sol) :");
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("error reading input");

    let ix = system_instruction::transfer(&usr_pubkey, &to_address, (amount.trim().parse::<f64>().unwrap() * LAMPORTS_PER_SOL as f64) as u64);
    let blockhash = rpc.get_latest_blockhash().expect("erreur récup blockhash");
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&usr_pubkey),
        &[&usr],
        blockhash,
    );

    println!("envoi de la tx");
    match rpc.send_and_confirm_transaction(&tx) {
        Ok(sig) => loop {
            if let Ok(confirmed) = rpc.confirm_transaction(&sig) {
                if confirmed {
                    println!("tx validée");
                    break;
                }
            }
        },
        Err(e) => println!("erreur dans la tx : {}", e),
    }
}

fn get_airdrop(rpc: &RpcClient, usr: &Keypair) {
    let usr_pubkey = Signer::pubkey(&usr);
    println!("airdrop 1 sol");
    match rpc.request_airdrop(&usr_pubkey, LAMPORTS_PER_SOL) {
        Ok(sig) => loop {
            if let Ok(confirmed) = rpc.confirm_transaction(&sig) {
                if confirmed {
                    println!("airdrop recu, tx : {}", sig);
                    break;
                }
            }
        },
        Err(e) => println!("erreur airdrop : {}", e),
    };
}

fn main() {
    let rpc = RpcClient::new_with_commitment("https://api.devnet.solana.com".to_string(), CommitmentConfig::confirmed(),);
    let usr = Keypair::new();
    let usr_pubkey = Signer::pubkey(&usr);

    loop {
        println!("Your address : {}", usr_pubkey.to_string());
        println!("{} sol", get_balance(&rpc, usr_pubkey));
        println!("1 : tx\n2 : request airdrop");
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("error reading input");
        let input = input_string.trim();
    
        if input == "1" {
            send_tx(&rpc, &usr);
        }
    
        else if input == "2" {
            get_airdrop(&rpc, &usr);
        }
    }

}