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

/*
 * 匹配到了长度, 但是是否结束
 * */
pub type IsEndCB = fn(c: &u8) -> bool;

impl<'a> Keyword<'a> {
    pub fn matched(&mut self, c: &u8, is_end_cb: IsEndCB) -> KeywordStatus {
        if self.src.len() == 0 {
            return KeywordStatus::Error;
        }
        if (is_end_cb)(c) {
            if self.index == self.length {
                self.index = 0;
                return KeywordStatus::Matched;
            }
        }
        if self.index >= self.length {
            self.index = 0;
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
                // self.index = 0;
                // return KeywordStatus::Error;
            }
        }
        return KeywordStatus::Continue;
    }

    pub fn last_result(&self) -> KeywordStatus {
        if self.index == self.length {
            return KeywordStatus::Matched;
        }
        KeywordStatus::Error
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
        let stmt = "funcx fun func function func".as_bytes();
        let mut func = Keyword::new("func".as_bytes());
        let mut function = Keyword::new("function".as_bytes());
        for c in stmt {
            match func.matched(c, |c| {
                if *c as char == ' ' {
                    return true;
                }
                false
            }) {
                KeywordStatus::Matched => {
                    println!("metched func");
                },
                _ => {
                }
            }
            match function.matched(c, |c| {
                if *c as char == ' ' {
                    return true
                }
                false
            }) {
                KeywordStatus::Matched => {
                    println!("matched function");
                },
                _ => {
                }
            }
        }
        if let KeywordStatus::Matched = func.last_result() {
            println!("metched func");
        };
        if let KeywordStatus::Matched = function.last_result() {
            println!("metched function");
        };
    }
}
