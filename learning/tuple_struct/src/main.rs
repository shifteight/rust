struct Color (u8, u8, u8);
struct Kilometers(i32);

fn main() {
	let black = Color (0, 0, 0);
	let Color (r, g, b) = black;
    println!("Black = rgb({},{}, {})", r, g, b);

    // newtype pattern
    let distance = Kilometers(20);
    let Kilometers(distance_in_km) = distance;
    println!("The distance: {} km", distance_in_km);
}
