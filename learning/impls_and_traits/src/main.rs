struct Player {
    first_name: String,
    last_name: String,
}

impl Player {

    fn new(first_name: String, last_name: String) -> Player {
        Player {
            first_name: first_name,
            last_name: last_name,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let player_1 = Player::new("Kevin".to_string(), "Qian".to_string());
    println!("Player 01: {}", player_1.full_name());
}
