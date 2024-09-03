mod debug;
mod tables;
mod io;
mod board;


fn main() {
    let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    board::Board::fen(fen);
}
