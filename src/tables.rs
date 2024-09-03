const LEN: usize = 64;

pub const FILE_A: u64 = 0x8080808080808080;
const FILE_B: u64 = 0x4040404040404040;
const FILE_C: u64 = 0x2020202020202020;
const FILE_D: u64 = 0x1010101010101010;
const FILE_E: u64 = 0x0808080808080808;
const FILE_F: u64 = 0x0404040404040404;
const FILE_G: u64 = 0x0202020202020202;
pub const FILE_H: u64 = 0x0101010101010101;

const RANK_1: u64 = 0x00000000000000FF;
const RANK_2: u64 = 0x000000000000FF00;
const RANK_3: u64 = 0x0000000000FF0000;
const RANK_4: u64 = 0x00000000FF000000;
const RANK_5: u64 = 0x000000FF00000000;
const RANK_6: u64 = 0x0000FF0000000000;
const RANK_7: u64 = 0x00FF000000000000;
const RANK_8: u64 = 0xFF00000000000000;

const fn generate_file_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;

    let files: [u64; 8] = [FILE_H, FILE_G, FILE_F, FILE_E, FILE_D, FILE_C, FILE_B, FILE_A];

    while i < LEN {
        table[i] = files[i%8] & !(1 << i);
        i += 1;
    }
    table
}

const fn generate_rank_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;

    let ranks: [u64; 8] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];

    while i < LEN {
        table[i] = ranks[i/8]  & !(1 << i);
        i += 1;
    }
    table
}

const fn generate_diag_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;

    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        let mut tmp: u64 = src;
        while tmp != 0 {
            tmp = (tmp >> 7) & !(FILE_H);
            bb |= tmp;
        }
        let mut tmp: u64 = src;
        while tmp != 0 {
            tmp = (tmp << 7) & !(FILE_A);
            bb |= tmp;
        }
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_antidiag_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;

    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        let mut tmp: u64 = src;
        while tmp != 0 {
            tmp = (tmp >> 9) & !(FILE_A);
            bb |= tmp;
        }
        let mut tmp: u64 = src;
        while tmp != 0 {
            tmp = (tmp << 9) & !(FILE_H);
            bb |= tmp;
        }
        table[i] = bb;
        i += 1;
    }
    table
}


const fn generate_bishop_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    
    const DIAG_TABLE: [u64; LEN] = generate_diag_table();
    const ANTIDIAG_TABLE: [u64; LEN] = generate_antidiag_table();
    
    while i < LEN {
        let mut bb: u64 = 0;
        bb |= DIAG_TABLE[i];
        bb |= ANTIDIAG_TABLE[i];
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_rook_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    
    const FILE_TABLE: [u64; LEN] = generate_file_table();
    const RANK_TABLE: [u64; LEN] = generate_rank_table();
    
    while i < LEN {
        let mut bb: u64 = 0;
        bb |= FILE_TABLE[i];
        bb |= RANK_TABLE[i];
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_queen_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    
    const DIAG_TABLE: [u64; LEN] = generate_diag_table();
    const ANTIDIAG_TABLE: [u64; LEN] = generate_antidiag_table();
    const FILE_TABLE: [u64; LEN] = generate_file_table();
    const RANK_TABLE: [u64; LEN] = generate_rank_table();
    
    while i < LEN {
        let mut bb: u64 = 0;
        bb |= FILE_TABLE[i];
        bb |= RANK_TABLE[i];
        bb |= DIAG_TABLE[i];
        bb |= ANTIDIAG_TABLE[i];
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_king_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        bb |= (src >> 9) & !(FILE_A);
        bb |=  src >> 8;
        bb |= (src >> 7) & !(FILE_H);
        bb |= (src >> 1) & !(FILE_A);
        bb |= (src << 1) & !(FILE_H);
        bb |= (src << 7) & !(FILE_A);
        bb |=  src << 8;
        bb |= (src << 9) & !(FILE_H);
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_knight_table() -> [u64;LEN] {
    let mut table: [u64;LEN] = [0;LEN];
    let mut i: usize = 0;
    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        bb |= (src >> 17) & !(FILE_A);
        bb |= (src >> 15) & !(FILE_H);
        bb |= (src >> 10) & !(FILE_A | FILE_B);
        bb |= (src >> 6)  & !(FILE_G | FILE_H);
        bb |= (src << 6)  & !(FILE_A | FILE_B);
        bb |= (src << 10) & !(FILE_G | FILE_H);
        bb |= (src << 15) & !(FILE_A);
        bb |= (src << 17) & !(FILE_H);
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_pawn_white_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        bb |= src << 8;
        if i >= 8 && i < 16 {
            bb |= src << 16;
        }
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_pawn_white_attack_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        bb |= (src << 7) & !(FILE_A);
        bb |= (src << 9) & !(FILE_H);
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_pawn_black_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        bb |= src >> 8;
        if i >= 48 && i < 56 {
            bb |= src >> 16;
        }
        table[i] = bb;
        i += 1;
    }
    table
}

const fn generate_pawn_black_attack_table() -> [u64; LEN] {
    let mut table: [u64; LEN] = [0; LEN];
    let mut i: usize = 0;
    while i < LEN {
        let src: u64 = 1 << i;
        let mut bb: u64 = 0;
        bb |= (src >> 7) & !(FILE_H);
        bb |= (src >> 9) & !(FILE_A);
        table[i] = bb;
        i += 1;
    }
    table
}

pub static FILE: [u64; 64] = generate_file_table();
pub static RANK: [u64; 64] = generate_rank_table();
pub static DIAG: [u64; 64] = generate_diag_table();
pub static ANTIDIAG: [u64; 64] = generate_antidiag_table();

pub static PAWN_WHITE: [u64; LEN] = generate_pawn_white_table();
pub static PAWN_WHITE_ATTACK: [u64; LEN] = generate_pawn_white_attack_table();
pub static PAWN_BLACK: [u64; LEN] = generate_pawn_black_table();
pub static PAWN_BLACK_ATTACK: [u64; LEN] = generate_pawn_black_attack_table();

pub static BISHOP: [u64; LEN] = generate_bishop_table();
pub static ROOK: [u64; LEN] = generate_rook_table();
pub static QUEEN: [u64; LEN] = generate_queen_table();

pub static KING: [u64; LEN] = generate_king_table();
pub static KNIGHT: [u64; LEN] = generate_knight_table();
