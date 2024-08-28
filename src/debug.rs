pub fn print_bb(bb: u64) {
    const LEN: u8 = 64; 
    for i in (0..LEN).rev() {
        let bit: u64 = bb & (1 << i);
        let b: u8 = if bit==0 {0} else {1};
        print!("{b} ");
        if i%8 == 0 {print!("\n")}
    }
}