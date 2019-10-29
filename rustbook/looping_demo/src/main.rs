fn main() {

    // print 0 to 10
    for x in 0..10 {
        println!("{}", x);
    }

    // enumerate on ranges
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    // enumerate on iterators
    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

    // print only odds bwtween 0 and 10
    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x);
    }
}