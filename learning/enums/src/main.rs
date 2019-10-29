#[derive(Debug)]
enum FlashMessage {
	Success,
	Warning {category: i32, message: String},
	Error(String)
}

fn main() {
	let mut form_status = FlashMessage::Success;
	print_flash_message(form_status);

	form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
	print_flash_message(form_status);

	form_status = FlashMessage::Error(String::from("Connection Error"));
	print_flash_message(form_status);
}

fn print_flash_message(m: FlashMessage) {
	match m {
		FlashMessage::Success => 
			println!("From submitted correctly"),
		FlashMessage::Warning {category, message} => 
			println!("Warning: {} - {}", category, message),
		FlashMessage::Error(msg) =>
			println!("Error: {}", msg)
	}
}