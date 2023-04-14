// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a hint.

use core::fmt::Display;
// I AM NOT DONE

// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    // TODO: Implement the function body.
    // *arg.as_mut() *= *arg.as_mut();
    *arg.as_mut() *= *arg.as_mut();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}

struct Document {
    info: String,
    content: Vec<u8>,
}

impl<T: ?Sized> AsMut<T> for Document
where
    Vec<u8>: AsMut<T>,
{
    fn as_mut(&mut self) -> &mut T {
        self.content.as_mut()
    }
}

fn caesar<T: AsMut<[u8]>>(data: &mut T, key: u8) {
    for byte in data.as_mut() {
        *byte = byte.wrapping_add(key);
    }
}

fn null_terminate<T: AsMut<Vec<u8>>>(data: &mut T) {
    // Using a non-generic inner function, which contains most of the
    // functionality, helps to minimize monomorphization overhead.
    fn doit(data: &mut Vec<u8>) {
        let len = data.len();
        if len == 0 || data[len - 1] != 0 {
            data.push(0);
        }
    }
    doit(data.as_mut());
}

fn main() {
    let mut v: Vec<u8> = vec![1, 2, 3];
    caesar(&mut v, 5);
    assert_eq!(v, [6, 7, 8]);
    null_terminate(&mut v);
    assert_eq!(v, [6, 7, 8, 0]);
    let mut doc = Document {
        info: String::from("Example"),
        content: vec![17, 19, 8],
    };
    caesar(&mut doc, 1);
    assert_eq!(doc.content, [18, 20, 9]);
    null_terminate(&mut doc);
    assert_eq!(doc.content, [18, 20, 9, 0]);
}
