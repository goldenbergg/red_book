// main.rs
//use rand::{Rng, SeedableRng};
//use rand_pcg::Pcg64;
//use red_book_libs::bitboards;
//use red_book_libs::defs;
use red_book_libs::init;

fn main() {
    init::all_init();

    /*
    let mut rng = Pcg64::seed_from_u64(1);
    let piece_one: i32 = rng.gen();
    let piece_two: i32 = rng.gen();
    let piece_three: i32 = rng.gen();
    let piece_four: i32 = rng.gen();
    println!("piece_one: {:#X}", piece_one);
    println!("piece_two: {:#X}", piece_two);
    println!("piece_three: {:#X}", piece_three);
    println!("piece_four: {:#X}", piece_four);

    let key: i32 = piece_one ^ piece_two ^ piece_four;
    let mut temp_key: i32 = piece_two;
    temp_key ^= piece_three;
    temp_key ^= piece_four;
    temp_key ^= piece_one;

    println!("key: {:#X}", key);
    println!("temp_key: {:#X}", temp_key);

    temp_key ^= piece_three;
    println!("temp_key: {:#X}", temp_key);

    temp_key ^= piece_three;
    println!("temp_key: {:#X}", temp_key);
    */

    /*
    let mut play_bit_board: u64 = 0u64;
    defs::setbit(&mut play_bit_board, 61);
    bitboards::print_bit_board(play_bit_board);
    defs::clrbit(&mut play_bit_board, 61);
    bitboards::print_bit_board(play_bit_board);
    */

    /*
    let mut index: i32 = 0 as i32;
	while index < 64 as i32 {
		println!("Index: {}", index);
        unsafe {
            bitboards::print_bit_board(defs::CLEAR_MASK[index as usize]);
        }
        println!();
		index += 1;
	}
    */

    /*
    play_bit_board |= 1u64 << defs::sq64(defs::Squares::D2 as usize);
    play_bit_board |= 1u64 << defs::sq64(defs::Squares::D3 as usize);
    play_bit_board |= 1u64 << defs::sq64(defs::Squares::D4 as usize);

    let mut sq64: i32;
    while play_bit_board != 0 {
        sq64 = defs::pop(&mut play_bit_board);
        println!("Popped: {}", sq64);
        bitboards::print_bit_board(play_bit_board);
    }
    */

    /*
    // Tests if bitboards.rs works properly
    print!("Start:");
    println!();
    println!();
    bitboards::print_bit_board(play_bit_board);
    play_bit_board |= 1u64 << defs::sq64(defs::Squares::D2 as usize);
    print!("D2 Added:");
    println!();
    println!();
    bitboards::print_bit_board(play_bit_board);
    play_bit_board |= 1u64 << defs::sq64(defs::Squares::G2 as usize);
    print!("G2 Added:");
    println!();
    println!();
    bitboards::print_bit_board(play_bit_board);
    */

    /*
    // Tests whether static mutable arrays are initialized properly
    for i in 0..defs::BRD_SQ_NUM {
        if i % 10 == 0 {
            println!();
        }
        unsafe {
            print!("{:5}", defs::SQ120_TO_SQ64[i as usize]);
        }
    }
    println!();
    println!();
    for i in 0..64 {
        if i % 8 == 0 {
            println!();
        }
        unsafe {
            print!("{:5}", defs::SQ64_TO_SQ120[i as usize]);
        }
    }
    println!();
    */
}