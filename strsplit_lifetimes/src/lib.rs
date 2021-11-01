//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// generally use anonymous lifetimes if you can.
// usually you dont need multiple lifetimes, quite rare, comes up when you need to store multiple
// references

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    // by specifing lifetime
    // we say that remainder and delimiter
    // live 'a long(the pointers are valid for that long).
    remainder: Option<&'haystack str>, 
    delimiter: D,
}

// str -> [char] (similar to)
// &str -> &[char], could point to anything, the stack, the heap, something in static memory
// String -> Vec<char>, heap allocated, dynamically expandable
//
// String -> &str (cheap -- AsRef)
// &str -> String (expensive -- memcpy)


impl<'haystack, D> StrSplit<'haystack, D> {
    // good to use Self type, so we dont have to change it
    // if we were to change the name of the struct
    
    // lifetime of reference outlives lifetime of borrowed content...
    // this happens because we said that StrSplit<'_> has this lifetime.
    // but our haystack and delimiter has a lifetime of whatever the lifetime of haystack: &str is.
    // these parameters could be deallocated immediately after we return from new, which
    // would make us have a StrSplit we invalid pointers since we cant gurantee that the lifetime
    // of haystack is as long as the lifetime of StrSplit.
    //
    // We now give you StrSplit with a lifetime of 'a, if you give me str pointers that are 'a 
    // the pointers that we give in can live as long as they want, but they have to at least live
    // 'a time.
    // This means that we can only use StrSplit as long as the input strings are still valid.
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
    
}

// use match if I can care about more than one pattern
// if I only care about one pattern, use "if let"

// can use anonymous lifetimes when the compiler can guess that there is only one other lifetime
// impl Foo {
//     fn get_ref(&self) -> &'_ str {}
//                ^ can use anonymous lifetime since the only other lifetime will be the reference
//                to self
// }


pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>; 
}

// let x: StrSplit;
// for part in x {
// }
impl<'haystack, D>  Iterator for StrSplit<'haystack, D> 
where 
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        // ref mut => We want a mutable reference to self.remainder if it is Some
        // I want a mutable reference to the thing I am matching rather than get the thing I am
        // matching itself
        if let Some(ref mut remainder /* &mut &'a str */) = self.remainder /* Option<&'a str> */ {
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

        //let rest = self.remainder;
        // We are allowed to set self.remainder to an empty string because
        // "" has the type of &'static str, and self.remainder has the lifetime of 'a, so 
        // since 'static lives till the end of the program, we can reduce that lifetime to the
        // lifetime of 'a, since 'static lives longer than 'a. This does not apply the other way around though.
        //self.remainder = ""; 
        //Some(rest)
    }
}


impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
} 
