use crate::util;
use crate::board_manip;
use crate::eval_search;
use crate::debug;

pub fn uci_handler(input: &str, board:util::BoardType) -> util::BoardType {

    let move_inp = input.split(" ").collect::<Vec<_>>();
    let source = &move_inp[move_inp.len() - 1][..2];
    let destination = &move_inp[move_inp.len() - 1][2..4];



    //add uci's move to the board
    let altered = board_manip::move_piece(util::string_square_to_int_parse(source),  util::string_square_to_int_parse(destination), board); 

    //to do: add computer's move to board

    return altered;
}
