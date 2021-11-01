//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit<'a> {
    // by specifing lifetime
    // we say that remainder and delimiter
    // live 'a long(the pointers are valid for that long).
    remainder: &'a str, 
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
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
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
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

// let x: StrSplit;
// for part in x {
// }
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO: bug
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}


#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]).into_iter();
}
