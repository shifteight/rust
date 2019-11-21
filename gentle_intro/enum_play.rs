#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }

    fn next(&self) -> Direction {
        use Direction::*;
        match *self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
}

fn eat_and_dum(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is {}", s),
        Bool(b) => println!("boolean is {}", b),
    }
}

fn dump(v: &Value) {
    use Value::*;
    match *v {
        Number(n) => println!("number is {}", n),
        Str(ref s) => println!("string is {}", s),
        Bool(b) => println!("boolean is {}", b),
    }
}

impl Value {
    fn to_str(self) -> Option<String> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }
}

fn main() {
    let start = Direction::Left;
    println!("start {:?}", start);
    assert_eq!(start, Direction::Left);

    let mut d = start;
    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }

    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);
    println!("n {:?} s {:?} b {:?}", n, s, b);
    eat_and_dum(n);
    //println!("{:?}", n);  // n is moved
    eat_and_dum(s);
    eat_and_dum(b);
    let s1 = Str("world".to_string());
    dump(&s1);
    println!("{:?}", s1);
    println!("s1? {:?}", s1.to_str());
    //println!("{:?}", s1);  // s1 is moved
}
