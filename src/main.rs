mod debug;
mod tables;
fn main() {
    for i in (0..64).rev() {
        println!("{i}");
        debug::print_bb(tables::PAWN_BLACK_ATTACK[i]);
        println!("");
    }
}
