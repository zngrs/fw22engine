use colored::Colorize;
pub type BoardType = [u64; 13]; // 13 as the first 12 are for piece locations, the last for board
// data, i.e., castling eligibility, who's to play, en passant, move
// repetition
pub type BoardData = u64;
pub type ZobristHash = u64;

pub const BINARY_NUMBERS: BoardType = [
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000000000,
];



pub fn console_padding ( lines: i32) {
    for _n in  0..lines + 1 {
        println!("");
    }
}

pub fn get_nth_digit(binary_number: u64, n: u64) -> u64 {
    if 2^n > binary_number {
        return 0;
    }
    (binary_number >> n) & 1

}



pub fn generate_board_hash ( board_to_hash: BoardType ) -> ZobristHash {
    let mut board_hash: u64 =0b0000000000000000000000000000000000000000000000000000000000000000; 
    return board_hash;
}

/*
0: white pawn
1: black pawn
2: white bishop
3: black bishop
4: white knight
5: black knight
6: white rook
7: black rook
8: white queen
9: black queen
10: white king
11: black king
12: board data
*/

pub const TEST_POS_FEN_1: &'static str = "r3r1k1/1p3nqp/2pp4/p4p2/Pn3P1Q/2N4P/1PPR2P1/3R1BK1";

pub const WHITE_PAWNS_BB: i8 = 0;
pub const BLACK_PAWNS_BB: i8 = 1;
pub const WHITE_BISHOPS_BB: i8 = 2;
pub const BLACK_BISHOPS_BB: i8 = 3;
pub const WHITE_KNIGTS_BB: i8 = 4;
pub const BLACK_KNIGHTS_BB: i8 = 5;
pub const WHITE_ROOK_BB: i8 = 6;
pub const BLACK_ROOK_BB: i8 = 7;
pub const WHITE_QUEEN_BB: i8 = 8;
pub const BLACK_QUEEN_BB: i8 = 9;
pub const WHITE_KING_BB: i8 = 10;
pub const BLACK_KING_BB: i8 = 11;

const PIECE_CHARS: [(char, char); 6] = [
    ('P', 'p'), // Pawns
    ('B', 'b'), // Bishops
    ('N', 'n'), // Knights
    ('R', 'r'), // Rooks
    ('Q', 'q'), // Queens
    ('K', 'k'), // Kings
];

pub fn log_board(mut board: &[u64]) {
    let mut board_representation = [['.'; 8]; 8]; // Initialize the board with empty squares
    // Iterate through each piece type bitboard
    board = board.split_last().unwrap().1;
    for (piece_idx, &bitboard) in board.iter().enumerate() {
        let (white_char, black_char) = PIECE_CHARS[piece_idx / 2]; // Get the piece characters

        for square in 0..64 {
            if (bitboard & (1u64 << square)) != 0 {
                let rank = square / 8;
                let file = square % 8;

                let piece_char = if piece_idx % 2 == 0 {
                    white_char
                } else {
                    black_char }; board_representation[rank][file] = piece_char }
        }
    }

    for rank in (0..8).rev() {
        print!("{}  ┃    ", 1 +  rank);

        for file in 0..8 {
            if board_representation[rank][file].to_string() != board_representation[rank][file].to_string().to_uppercase() {

                print!("{}  ",  board_representation[rank][file].to_string().green().bold());
            } else {

                print!("{}  ",  board_representation[rank][file].to_string().white().bold());
            }

        }

        console_padding(1);
    }
    println!("{}", "   ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━".blue());
    println!("        A  B  C  D  E  F  G  H");
    console_padding(1);
}

pub fn fen_to_bitboard(fen: &str) -> [u64; 13] {
    let mut bitboards: [u64; 13] = [0; 13];
    // Split the FEN string by spaces (the first part represents the board)
    let parts: Vec<&str> = fen.split(' ').collect();


    // The first part of the FEN is the board, split it into rows (8 rows)
    let rows: Vec<&str> = parts[0].split('/').collect();

    // At this point, `rows` contains the 8 rows as strings from the FEN notation.

    for (rank, row) in rows.iter().enumerate() {
        let mut file = 0;
        for char in row.chars() {
            if char.is_digit(10) {
                file += char.to_digit(10).unwrap() as usize;
            } else {
                let bit_index = (7 - rank) * 8 + file; // Calculate bitboard index (0 - 63)

                match char {
                    'P' => bitboards[WHITE_PAWNS_BB as usize] |= 1 << bit_index,
                    'p' => bitboards[BLACK_PAWNS_BB as usize] |= 1 << bit_index,
                    'N' => bitboards[WHITE_KNIGTS_BB as usize] |= 1 << bit_index,
                    'n' => bitboards[BLACK_KNIGHTS_BB as usize] |= 1 << bit_index,
                    'B' => bitboards[WHITE_BISHOPS_BB as usize] |= 1 << bit_index,
                    'b' => bitboards[BLACK_BISHOPS_BB as usize] |= 1 << bit_index,
                    'R' => bitboards[WHITE_ROOK_BB as usize] |= 1 << bit_index,
                    'r' => bitboards[BLACK_ROOK_BB as usize] |= 1 << bit_index,
                    'Q' => bitboards[WHITE_QUEEN_BB as usize] |= 1 << bit_index,
                    'q' => bitboards[BLACK_QUEEN_BB as usize] |= 1 << bit_index,
                    'K' => bitboards[WHITE_KING_BB as usize] |= 1 << bit_index,
                    'k' => bitboards[BLACK_KING_BB as usize] |= 1 << bit_index,
                    _ => ()



                }
                file += 1;
            }

        }

    }

    //board data, ie. castling eligibility, current player
    if parts[1] == "b" { // who's to play, 1 / true = black
        bitboards[12] = set_nth_bit(bitboards[12], 0, true);
    }
    if parts[2].contains('Q') {
        bitboards[12] = set_nth_bit(bitboards[12], 1, true);
    }

    if parts[2].contains('K') {
        bitboards[12] = set_nth_bit(bitboards[12], 2, true);
    }
    if parts[2].contains('q') {
        bitboards[12] = set_nth_bit(bitboards[12], 3, true);
    }
    if parts[2].contains('k') {
        bitboards[12] = set_nth_bit(bitboards[12], 4, true);
    }

    return bitboards
}

pub fn set_nth_bit(x: u64, n: usize, value: bool) -> u64 {
    if value {
        x | (1 << n)  // Set the nth bit to 1
    } else {
        x & !(1 << n) // Set the nth bit to 0
    }
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub fn string_square_to_int_parse(square_str: &str) -> u64 {
    let mut i: u64 = 0;
    let mut chars = square_str.chars();

    match chars.next() {
        Some('b') => i += 1,
        Some('c') => i += 2,
        Some('d') => i += 3,
        Some('e') => i += 4, // Adjusted to correct values
        Some('f') => i += 5,
        Some('g') => i += 6,
        Some('h') => i += 7,
        _ => i += 0, // Default case if no match
    }

    if let Some(digit_char) = chars.next() {
        if let Some(digit) = digit_char.to_digit(10) {
            // Convert the row number to an integer and adjust for 0-based index
            i += (digit - 1) as u64 * 8;
        } else {
            return 13; // Return None if the second character isn't a digit
        }
    } else {
        return 13; // Return None if the string is too short
    }

    return i

}

