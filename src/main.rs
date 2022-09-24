use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0;32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    value: 50,
                    to_addr: "Alice".to_owned(),
                },
                transaction::Output {
                    value: 7,
                    to_addr: "Bob".to_owned(),
                }
            ],
        }],
        difficulty
    );

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = BlockChain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block!");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 300,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 36,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    }
                ],
            }
        ],
        difficulty
    );

    block.mine();

    println!("Mined new block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add new block!");
}