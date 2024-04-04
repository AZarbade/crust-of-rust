pub struct StrSplit<'heystack, D> {
    remainder: Option<&'heystack str>,
    delimiter: D,
}

impl<'heystack, D> StrSplit<'heystack, D> {
    pub fn new(heystack: &'heystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(heystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

// WARNING: dont understand this block
impl<'heystack, D> Iterator for StrSplit<'heystack, D>
where
    D: Delimiter,
{
    type Item = &'heystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }

        // NOTE: below code is for documentaion purpose only!

        // if let Some(next_delim) = self.remainder.find(self.delimiter) {
        //     let until_delimiter = &self.remainder[..next_delim];
        //     self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
        //     Some(until_delimiter)
        // } else if self.remainder.is_empty() {
        //     // BUG: check above implementation for solution
        //     None
        // } else {
        //     let rest = self.remainder;
        //     self.remainder = "";
        //     // WARNING: why? &'a str is okay with &'static str
        //     // in the sense of decision making. why resort to inference
        //     // when spent so much in making it verbose?
        //     //
        //     // NOTE: any string that is defined as a raw (i.e. inside "")
        //     // is complied within the binary itself. Hence, it will always
        //     // my present in the program for complete runtime. Therefore, it
        //     // has by default a 'static lifetime.
        //     Some(rest)
        // }
    }
}

#[test]
fn it_works() {
    let heystack = "a:b:c:d:e";
    let letters = StrSplit::new(heystack, ":");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"]));
}

#[test]
fn tail() {
    let heystack = "a:b:c:d:";
    let letters = StrSplit::new(heystack, ":");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""]));
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}
