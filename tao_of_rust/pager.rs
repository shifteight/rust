trait Page {
	fn set_page(&self, p: i32) {
		println!("Page Default: 1");
	}
}

trait PerPage {
	fn set_perpage(&self, num: i32) {
		println!("Per Page Default: 10");
	}
}

struct MyPaginate {
	page: i32
}

trait Paginate: Page + PerPage {
	fn set_skip_page(&self, num: i32) {
		println!("Skip page: {:?}", num);
	}
}

impl Page for MyPaginate {}
impl PerPage for MyPaginate {}
impl <T: Page + PerPage> Paginate for T {}

fn main() {
	let my_paginate = MyPaginate {page: 1};
	my_paginate.set_page(2);
	my_paginate.set_perpage(100);
	my_paginate.set_skip_page(12);
}