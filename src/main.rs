use red_book_libs::defs;
use red_book_libs::init;
fn main() {
    init::all_init();
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
}
