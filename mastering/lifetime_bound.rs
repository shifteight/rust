enum Level {
    Error,
}

struct Logger<'a>(&'a str, Level);

fn configure_logger<T>(_t: T)
where
    T: Send + 'static,
{
    // configure the logger here
}

fn main() {
    let name = "Global";
    let log1 = Logger(name, Level::Error);
    configure_logger(log1);
    let other = String::from("Local");
    let log2 = Logger(&other, Level::Error);
    configure_logger(log2);
}
