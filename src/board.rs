#[path = "types.rs"] mod types;
#[path = "tables.rs"] mod tables;

use num_traits::{FromPrimitive, ToPrimitive};

fn sq_to_index(sq: String) -> u64{
    let file = match sq.chars().next().unwrap() {
        'a' => 7,    
        'b' => 6,
        'c' => 5,
        'd' => 4,
        'e' => 3,
        'f' => 2,
        'g' => 1,
        'h' => 0,
        _ => panic!("bad")
    };
    let rank = match sq.chars().next().unwrap() {
        '8' => 7,    
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        '1' => 0,
        _ => panic!("bad")
    };
    file + rank*8
}

fn piece_to_char(color: types::Color, piece: types::Piece) -> (char) {
    match color {
        types::Color::WHITE => {
            match piece {
                Piece::PAWN   => 'P',
                Piece::KNIGHT => 'N',
                Piece::BISHOP => 'B',
                Piece::ROOK   => 'R',
                Piece::QUEEN  => 'Q',
                Piece::KING   => 'K',
            }
        },
        types::Color::BLACK => {
            match piece {
                types::Piece::PAWN   => 'p',
                types::Piece::KNIGHT => 'n',
                types::Piece::BISHOP => 'b',
                types::Piece::ROOK   => 'r',
                types::Piece::QUEEN  => 'q',
                types::Piece::KING   => 'k',
            }
        }
    }
}

fn char_to_piece(c: char) -> (types::Color, types::Piece) {
    match c {
        'P' => (types::Color::WHITE, types::Piece::PAWN),
        'N' => (types::Color::WHITE, types::Piece::KNIGHT),
        'B' => (types::Color::WHITE, types::Piece::BISHOP),
        'R' => (types::Color::WHITE, types::Piece::ROOK),
        'Q' => (types::Color::WHITE, types::Piece::QUEEN),
        'K' => (types::Color::WHITE, types::Piece::KING),
        'p' => (types::Color::BLACK, types::Piece::PAWN),
        'n' => (types::Color::BLACK, types::Piece::KNIGHT),
        'b' => (types::Color::BLACK, types::Piece::BISHOP),
        'r' => (types::Color::BLACK, types::Piece::ROOK),
        'q' => (types::Color::BLACK, types::Piece::QUEEN),
        'k' => (types::Color::BLACK, types::Piece::KING),
        _ => panic!("Invalid match making in char_to_pieces")
    }
}

fn print_board(bbs:[[u64; 6]; 2]) {
    'outer: for i in (0..64).rev() {
        let mask:u64 = 1 << i;
        for n in 0..2 {
            for m in 0..6 {
                if bbs[n][m] & mask != 0 {
                    let color: types::Color = <types::Color as FromPrimitive>::from_usize(n).unwrap();
                    let piece: types::Piece = <types::Piece as FromPrimitive>::from_usize(m).unwrap();
                    let c: char = piece_to_char(color, piece);
                    print!("{c} ");
                    if i % 8 == 0 {
                        print!("\n");
                    }
                    continue 'outer;
                }
            }
        }
        print!(". ");
        if i % 8 == 0 {
            print!("\n");
        }
    }
}

pub struct Board {
    bitboards: [[u64; 6]; 2],
    player: types::Color,
    castling: (bool, bool, bool, bool), //K, Q, k, q
    enpassant: u64,
    halfmove_counter: u8,
    move_counter: u8,

    not_player: types::Color,

    occ_opponent: u64,
    occ_player: u64,
    occ: u64
}

impl Board {
    pub fn fen(fen: String) -> Board {
        fen_to_board(fen)
    }
    pub fn new() -> Board {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        Board::fen(fen)
    }
    pub fn get_moves(&self) -> Vec<types::Move> {
        // calculating helper values
        let white_occ = 0u64;
        for n in 0..2 {
            for m in 0..6 {
                
            }
        }

        let mut moves: Vec<types::Move> = Vec::<types::Move>::new();
        moves.append(&mut self.get_moves_pawn());
        moves.append(&mut self.get_moves_knight());
        moves.append(&mut self.get_moves_bishop());
        moves.append(&mut self.get_moves_rook());
        moves.append(&mut self.get_moves_queen());
        moves.append(&mut self.get_moves_king());

        moves
    }
    //pub fn move(move:Move) {
    //}

    fn get_moves_pawn(&self) -> Vec<types::Move> {
        match self.player {
            types::Color::WHITE => self.get_moves_pawn_white(),
            types::Color::BLACK => self.get_moves_pawn_black()
        }
    }

    fn get_moves_pawn_white(&self) -> Vec<types::Move> {
        let mut moves: Vec<types::Move> = Vec::<types::Move>::new();
        for i in 0..64 {
            if self.bitboards[self.player as usize][types::Piece::PAWN as usize] & 1 << i != 0 {
                let dst: u64 = i + 8;
                if i << 8 & !self.occ != 0 {
                    if i >= 56 {
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::KNIGHT));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::BISHOP));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::ROOK));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::QUEEN));
                    }
                    let dst: u64 = i+16;
                    if (i >= 8) && (i < 16) {
                        moves.push(types::Move::new(i as u8, dst as u8));
                    }
                }
                if i >= 56 {
                    let dst = i+7;
                    if (i << 7 & !tables::FILE_A) & self.occ_opponent != 0 {
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::KNIGHT));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::BISHOP));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::ROOK));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::QUEEN));
                    }
                    let dst = i+9;
                    if (i << 9 & !tables::FILE_H) & self.occ_opponent != 0 {
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::KNIGHT));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::BISHOP));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::ROOK));
                        moves.push(types::Move::promotion(i as u8, dst as u8, types::Piece::QUEEN));
                    }
                }else if (i << 7 & !tables::FILE_A) & self.occ_opponent != 0 {
                    let dst = i+7;
                    moves.push(types::Move::new(i as u8, dst as u8));
                }else if (i << 9 & !tables::FILE_H) & self.occ_opponent != 0 {
                    let dst = i+9;
                    moves.push(types::Move::new(i as u8, dst as u8));
                }
            }
        }
        moves
    }

    fn get_moves_knight(&self) -> Vec<types::Move> {
        let mut moves: Vec<types::Move> = Vec::<types::Move>::new();
        for i in 0..64 {
            
        }
    }

    fn get_moves_bishop(&self) -> Vec<types::Move> {
        
    }

    fn get_moves_rook(&self) -> Vec<types::Move> {
        
    }

    fn get_moves_queen(&self) -> Vec<types::Move> {
        
    }

    fn get_moves_king(&self) -> Vec<types::Move> {

    }
}

pub fn fen_to_board(fen:String) -> Board {

    // example fen:
    // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"

    let mut arr: Vec<&str> = fen.split(" ").collect();

    let bb_str: String = arr.remove(0).replace('/', "");
    let mut i = 0;
    let mut bbs: [[u64; 6]; 2] = [[0; 6]; 2];
    const nums: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

    // match bbs
    for c in bb_str.chars().rev() {
        if nums.contains(&c) {
            let num = c.to_digit(10).unwrap();
            i += num;
        } else {
            let (color, piece) = char_to_piece(c);
            let color = color as usize;
            let piece = piece as usize;
            bbs[color][piece] |= 1 << i;
            i += 1;
        }
    }

    // match player_to_move
    let c: &str = arr.remove(0);
    let player: types::Color = {
            match c.chars().next().unwrap() {
            'w' => Color::WHITE,
            'b' => Color::BLACK,
            _ => panic!("Invalid player_to_move in fen")
        }
    };

    // match castling rights
    let castling = arr.remove(0);
    let castling_white_king: bool = castling.contains('K');
    let castling_white_queen: bool = castling.contains('Q');
    let castling_black_king: bool = castling.contains('k');
    let castling_black_queen: bool = castling.contains('q');
    let castling = (castling_white_king, castling_white_queen, castling_black_king, castling_black_queen);

    let enpassant_str = arr.remove(0);
    let enpassant: u64 = if enpassant_str.contains('-') {0u64} else {1 << sq_to_index(String::from(enpassant_str))};

    let halfmove_counter: u8 = arr.remove(0).parse::<u8>().unwrap();

    let move_counter: u8 = arr.remove(0).parse::<u8>().unwrap();

    Board {
        bitboards: bbs,
        player: player,
        castling: castling,
        enpassant: enpassant,
        halfmove_counter: halfmove_counter,
        move_counter: move_counter,
        white_occ: 0u64,
        black_occ: 0u64,
        occ = 0u64
    }
}