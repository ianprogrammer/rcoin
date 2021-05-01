use rcoinlib::*;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_new_transaction(Transaction {
        sender: String::from("Ryan"),
        receiver: String::from("Dan"),
        amount: 2000.0,
    });
    blockchain.mine_unined_transactions("First Miner".to_owned());

    println!("{}", blockchain.is_valid_chain());

    println!("{:#?}", blockchain);
}
