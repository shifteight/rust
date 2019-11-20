# Basics

## Strings

The borrow operator can coerce String into &str, just as Vec<T> could be coerced into &[T].

Under the hood, String is basically a Vec<u8> and &str is &[u8], but those bytes must represent valid UTF-8 text.
