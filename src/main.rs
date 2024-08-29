mod util;
mod board_manip;
mod debug;
mod test_suite;
mod uci;
mod eval_search;
mod move_gen;

fn main() {

    util::console_padding(1);
    println!("fw22 Engine");
    util::console_padding(2);
    println!(r"   |\_
  /  .\_
 |   ___)
 |    \
 |  =  |
 /_____\
[_______]");

    util::console_padding(1);
    let mut board_state = util::fen_to_bitboard(test_suite::START_POS_FEN);



    let mut game_ongoing: bool = true;

    while game_ongoing {

        util::log_board(&board_state);
        let mut input_move = String::new();
        let _inp = std::io::stdin().read_line(&mut input_move).unwrap();
        util::trim_newline(&mut input_move);
        match input_move.as_str() {
            "quit" => game_ongoing = false,
            "uci" => println!("uciok"),
            "isready" => println!("readyok"),
            &_ => board_state = uci::uci_handler(&input_move, board_state),
        }
    }
}
