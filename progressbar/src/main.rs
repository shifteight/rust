use indicatif;

use std::{thread, time};



fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn do_hard_work() {
	let ten_millis = time::Duration::from_millis(10);

	std::thread::sleep(ten_millis);
}