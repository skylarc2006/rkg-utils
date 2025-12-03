pub mod rkg;

use crate::rkg::header::finish_time::FinishTime;

fn main() {
    let m: u8 = 1;
    let s: u8 = 37;
    let ms: u16 = 467;

    let finish_time: FinishTime = FinishTime::new(m,s, ms);
    println!("Finish time: {}", finish_time.string());
}
