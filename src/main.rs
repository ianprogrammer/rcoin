use ed25519_dalek::Keypair;
use rcoinlib::*;

fn main() {
    let mut blockchain = Blockchain::new();

    let ian_key = Wallet::new();
    let cyn_key = Wallet::new();

    let miner_key = Wallet::new();

    let mut first_transaction = Transaction {
        sender: Some(ian_key.public),
        receiver: Some(cyn_key.public),
        amount: 1500.0,
        signature: None,
    };

    first_transaction.sign_transaction(Keypair {
        public: ian_key.public,
        secret: ian_key.secret,
    });

    blockchain.add_new_transaction(first_transaction);
    blockchain.mine_unined_transactions(miner_key.public);

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);
}
