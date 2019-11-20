#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
    	    first_name: first.to_string(),
    	    last_name: last.to_string()
	    }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn copy(&self) -> Self {
        Self::new(&self.first_name, &self.last_name)
    }
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
fn main() {
	let p = Person::new("John", "Smith");
	println!("person {} {}", p.first_name, p.last_name);
	println!("fullname {}", p.full_name());
	let mut q = p.copy();
	println!("{:?}", q);
	q.set_first_name("Jane");
	println!("{:?}", q.to_tuple());
	
}