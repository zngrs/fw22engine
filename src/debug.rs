pub fn log_bb_b2(board: [u64; 13]) {
    for i in 0..13 {
        println!("{}: {:b}",i, board[i]);
    }
}
