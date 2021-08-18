// main.rs
use red_book_libs::attack;
//use red_book_libs::bitboards;
//use red_book_libs::board;
use red_book_libs::defs;
use red_book_libs::init;
use red_book_libs::io;

pub fn print_bin(pc_move: i32) {
    let mut index: i32;
    println!("As binary:");
    index = 27;
    while index >= 0 {
        if ((1 << index) & pc_move) > 0 {
            print!("1");
        }
        else {
            print!("0");
        }
        if (index != 28) && (index % 4 == 0) {
            print!(" ");
        }
        index -= 1;
    }
    println!();
}

pub fn show_sq_at_by_side(side: i32, pos: *const defs::SBoard) {
    let mut rank: i32;
    let mut file: i32;
    let mut sq: i32;
    println!();
    println!();
    unsafe { println!("Squares attacked by: {}", defs::SIDE_CHAR.chars().nth(side as usize).unwrap()); }
    rank = defs::Rank::Rank8 as i32;
    while rank >= (defs::Rank::Rank1 as i32) {
        file = defs::File::FileA as i32;
        while file <= (defs::File::FileH as i32) {
            sq = defs::fr2_sq(file, rank);
            if attack::sq_attacked(sq, side, pos) == (defs::TF::True as i32) {
                print!("X");
            }
            else {
                print!("-");
            }
            file += 1;
        }
        println!();
        rank -= 1;
    }
    println!();
    println!();
}

fn main() {
    init::all_init();
    let pc_move: i32;
    let from: i32 = defs::Squares::A2 as i32;
    let to: i32 = defs::Squares::H7 as i32;
    let cap: i32 = defs::Pieces::WR as i32;
    let prom: i32 = defs::Pieces::BK as i32;
    pc_move = (from) | (to << 7) | (cap << 14) | (prom << 20);

    println!("from: {} to: {} cap: {} prom: {}", defs::from_sq(pc_move), defs::to_sq(pc_move), defs::captured(pc_move), defs::promoted(pc_move));
    println!("Algebraic from: {}", io::pr_sq(from));
    println!("Algebraic to: {}", io::pr_sq(to));
    println!("Algebraic move: {}", io::pr_move(pc_move));
}

    /*
    let board = defs::SBoard {
        pieces: [100i32; 120],
        pawns: [0u64; 3],
        king_sq: [99i32; 2],
        side: 2i32,
        enpas: 99i32,
        fifty_move: 0i32,
        ply: 0i32,
        his_ply: 0i32,
        castle_perm: 0i32,
        pos_key: 0u64,
        pce_num: [0i32; 13],
        big_pce: [0i32; 2],
        maj_pce: [0i32; 2],
        min_pce: [0i32; 2],
        material: [0i32; 2],
        p_list: [[0i32; 10] ; 13],
    };
    let mut board1: [defs::SBoard; 1] = [board; 1];
    let fen4: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    board::parse_fen(fen4, &mut board1[0]);
    board::print_board(&board1[0]);
    assert!(board::check_board(&board1[0]) == (defs::TF::True as i32));
    */

    /*
    let board = defs::SBoard {
        pieces: [100i32; 120],
        pawns: [0u64; 3],
        king_sq: [99i32; 2],
        side: 2i32,
        enpas: 99i32,
        fifty_move: 0i32,
        ply: 0i32,
        his_ply: 0i32,
        castle_perm: 0i32,
        pos_key: 0u64,
        pce_num: [0i32; 13],
        big_pce: [0i32; 2],
        maj_pce: [0i32; 2],
        min_pce: [0i32; 2],
        material: [0i32; 2],
        p_list: [[0i32; 10] ; 13],
    };
    let mut board1: [defs::SBoard; 1] = [board; 1];
    let fen4: &str = "8/3q1p2/8/5P2/4Q3/8/8/8 w - - 0 2 ";
    board::parse_fen(fen4, &mut board1[0]);
    board::print_board(&board1[0]);
    //assert!(board::check_board(&board1[0]) == (defs::TF::True as i32));
    println!();
    println!();
    println!("White Attacking:");
    show_sq_at_by_side(defs::Colors::White as i32, &board1[0]);
    show_sq_at_by_side(defs::Colors::Black as i32, &board1[0]);
    */

    /*
    let board = defs::SBoard {
        pieces: [100i32; 120],
        pawns: [0u64; 3],
        king_sq: [99i32; 2],
        side: 2i32,
        enpas: 99i32,
        fifty_move: 0i32,
        ply: 0i32,
        his_ply: 0i32,
        castle_perm: 0i32,
        pos_key: 0u64,
        pce_num: [0i32; 13],
        big_pce: [0i32; 2],
        maj_pce: [0i32; 2],
        min_pce: [0i32; 2],
        material: [0i32; 2],
        p_list: [[0i32; 10] ; 13],
    };
    let mut board1: [defs::SBoard; 1] = [board; 1];
    let fen4: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    board::parse_fen(fen4, &mut board1[0]);
    board::print_board(&board1[0]);
    assert!(board::check_board(&board1[0]) == (defs::TF::True as i32));
    println!();
    println!("Forced asserts...");
    println!();
    //board1[0].pce_num[defs::Pieces::WP as i32 as usize] -= 1;
    unsafe { board1[0].pos_key ^= defs::SIDE_KEY; }
    assert!(board::check_board(&board1[0]) == (defs::TF::True as i32));
    */

    /*
    let board = defs::SBoard {
        pieces: [100i32; 120],
        pawns: [0u64; 3],
        king_sq: [99i32; 2],
        side: 2i32,
        enpas: 99i32,
        fifty_move: 0i32,
        ply: 0i32,
        his_ply: 0i32,
        castle_perm: 0i32,
        pos_key: 0u64,
        pce_num: [0i32; 13],
        big_pce: [0i32; 2],
        maj_pce: [0i32; 2],
        min_pce: [0i32; 2],
        material: [0i32; 2],
        p_list: [[0i32; 10] ; 13],
    };
    let mut board1: [defs::SBoard; 1] = [board; 1];
    let fen4: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    board::parse_fen(fen4, &mut board1[0]);
    board::print_board(&board1[0]);
    println!();
    println!("WP:");
    bitboards::print_bit_board(board1[0].pawns[defs::Colors::White as usize]);
    println!();
    println!("BP:");
    bitboards::print_bit_board(board1[0].pawns[defs::Colors::Black as usize]);
    println!();
    println!("All P:");
    bitboards::print_bit_board(board1[0].pawns[defs::Colors::Both as usize]);
    */

    /*
    let board = defs::SBoard {
        pieces: [100i32; 120],
        pawns: [0u64; 3],
        king_sq: [99i32; 2],
        side: 2i32,
        enpas: 99i32,
        fifty_move: 0i32,
        ply: 0i32,
        his_ply: 0i32,
        castle_perm: 0i32,
        pos_key: 0u64,
        pce_num: [0i32; 13],
        big_pce: [0i32; 2],
        maj_pce: [0i32; 2],
        min_pce: [0i32; 2],
        material: [0i32; 2],
        p_list: [[0i32; 10] ; 13],
    };
    let mut board1: [defs::SBoard; 1] = [board; 1];
    let fen1: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let fen2: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
    let fen3: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    let fen4: &str = "r1q1brk1/ppp2ppp/3b1n2/3p1N2/8/1PNQP3/PBP2PPP/3R1RK1 w - - 10 14";
    board::parse_fen(defs::START_FEN, &mut board1[0]);
    board::print_board(&board1[0]);
    board::parse_fen(fen1, &mut board1[0]);
    board::print_board(&board1[0]);
    board::parse_fen(fen2, &mut board1[0]);
    board::print_board(&board1[0]);
    board::parse_fen(fen3, &mut board1[0]);
    board::print_board(&board1[0]);
    board::parse_fen(fen4, &mut board1[0]);
    board::print_board(&board1[0]);
    */
    
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