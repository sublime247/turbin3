use solana_sdk::{message::Message, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, system_program, transaction::Transaction};
use solana_client::rpc_client::RpcClient; 
const RPC_URL: &str = "https://api.devnet.solana.com";
use solana_program::{system_instruction::transfer, };
use std::str::FromStr;
mod programs;
use crate::programs::Turbin3_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};
use bs58;
use std::io::{self, BufRead};


#[cfg(test)] mod tests {
    // use solana_sdk;
    #[test]
    fn keygen() {} #[test] fn airdop() {} #[test] fn transfer_sol() {} #[test] fn oo() {}
    }
  
    // let kp = Keypair::new();

    #[test]
fn keygen() {

// Create a new keypair
let kp = Keypair::new();
println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
println!("To save your wallet, copy and paste the following into a JSON file:");

println!("{:?}", kp.to_bytes());

}

#[test]
fn airdop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    // let client = RpcClient::new(RPC_URL);

    let client = RpcClient::new(RPC_URL); 
    // Finally, we're going to call the airdrop function:

// We're going to claim 2 devnet SOL tokens (2 billion lamports)
match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

Ok(s) => {
println!("Success! Check out your TX here:");

println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());

},

Err(e) => println!("Oops, something went wrong: {}", e.to_string()) };
}

#[test]
fn transfer_sol(){
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let to_pubkey = Pubkey::from_str("AYF7S4E7uP5uZWbwdwPhatytWJZaSq1AsZAvX7rgzpoW").unwrap();
    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");
    // let transaction = Transaction::new_signed_with_payer( &[transfer(

    //     &keypair.pubkey(), &to_pubkey, 1_000_000
        
    //     )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash
        
    //     );
        // Send the transaction
// let signature = rpc_client
// .send_and_confirm_transaction(&transaction)
// .expect("Failed to send transaction");
// // If everything went as planned, we'll print a link to the TX out to the terminal

// // Print our transaction out
//  println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

 let balance = rpc_client
.get_balance(&keypair.pubkey())
.expect("Failed to get balance");

// Create a test transaction to calculate fees
let message = Message::new_with_blockhash(

    &[transfer( &keypair.pubkey(), &to_pubkey, balance,
    
    )], Some(&keypair.pubkey()), &recent_blockhash
    
    );

    let fee= rpc_client

.get_fee_for_message(&message) .expect("Failed to get fee calculator");

let transaction =
Transaction::new_signed_with_payer(

&[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,

)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

let signature = rpc_client
.send_and_confirm_transaction(&transaction)
.expect("Failed to send transaction");

println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}


#[test]
fn oo(){
    let rpc_client = RpcClient::new(RPC_URL);
    let wallet = bs58::decode("My pub key").into_vec().unwrap();
     println!("{:?}", wallet);
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq",
signer.pubkey().to_bytes().as_ref()]);

let args = CompleteArgs {
 github: b"sublime247".to_vec() };

//  Get recent blockhash
let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent
blockhash");
// Now we need to populate our complete function:

// Now we can invoke the "complete" function 
let transaction =WbaPrereqProgram::complete(

&[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()),
&[&signer],

blockhash );

// Finally, we publish our transaction! 

let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed
to send transaction");

// Print our transaction out 
println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}