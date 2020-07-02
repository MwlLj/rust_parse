/*
 * 缓存输入, 并每次进行是否和输入匹配
 * */
pub struct Keyword<'a> {
    src: &'a [u8],
    index: usize,
    length: usize
}

pub enum KeywordStatus {
    Error,
    Continue,
    Matched
}

impl<'a> Keyword<'a> {
    pub fn matched(&mut self, c: &u8) -> KeywordStatus {
        if self.src.len() == 0 {
            return KeywordStatus::Error;
        }
        match self.src.get(self.index) {
            Some(v) => {
                if v == c {
                    self.index += 1;
                } else {
                    self.index = 0;
                    return KeywordStatus::Error;
                }
            },
            None => {
                panic!("should not happend");
            }
        }
        if self.index == self.length {
            self.index = 0;
            return KeywordStatus::Matched;
        }
        return KeywordStatus::Continue;
    }

    pub fn new(src: &'a [u8]) -> Self {
        Self{
            src: src,
            index: 0,
            length: src.len()
        }
    }
}

mod test {
    use super::*;

    #[test]
    #[ignore]
    fn keyword_matched_test() {
        let stmt = "hello {}, nice to meet too".as_bytes();
        let mut keyword = Keyword::new("{}".as_bytes());
        for c in stmt {
            match keyword.matched(c) {
                KeywordStatus::Matched => {
                    println!("metched");
                },
                _ => {
                }
            }
        }
    }
}
