trait GetSound {
    fn get_sound(&self) -> String;
}

struct Cat {
    sound: String,
}

impl GetSound for Cat {
    fn get_sound(&self) -> String {
        self.sound.clone()
    }
}

struct Bell {
    sound: String,
}

impl GetSound for Bell {
    fn get_sound(&self) -> String {
        self.sound.clone()
    }
}

fn make_sound<T: GetSound>(t: &T) {
    println!("{}!", t.get_sound())
}

fn main() {
    let kitty = Cat { sound: "Meow".to_string() };
    let the_bell = Bell { sound: "Ding Dong".to_string() };
    make_sound(&kitty);
    make_sound(&the_bell);
}