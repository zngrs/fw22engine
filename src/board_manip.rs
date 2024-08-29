use crate::util;
pub fn get_piece_at(square: u64, board: util::BoardType) -> u64 {
    for i in 0..12 {
        if util::get_nth_digit(board[i], square) == 1 {
            return i as u64;
        }
    }
    return 13;  // No piece found
}

pub fn move_piece(origin: u64, destination: u64, board: util::BoardType) -> util::BoardType{
    let mut b = board.clone();
    let moving_piece_type = get_piece_at(origin, board);
    let destination_piece_type = get_piece_at(destination, board);
    //remove prior piece in destination;
    if destination_piece_type != 13 {
    b[destination_piece_type as usize] = util::set_nth_bit(b[destination_piece_type as usize], destination as usize, false); 
    }
    //move moving piece;
    b[moving_piece_type as usize] = util::set_nth_bit(util::set_nth_bit(b[moving_piece_type as usize], origin as usize, false), destination as usize, true);
    return b;


}
