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
    pub fn matched(&mut self, c: &u8, is_end_cb: Option<IsEndCB>) -> KeywordStatus {
        if self.src.len() == 0 {
            return KeywordStatus::Error;
        }
        match &is_end_cb {
            Some(cb) => {
                if (cb)(c) {
                    if self.index == self.length {
                        self.index = 0;
                        return KeywordStatus::Matched;
                    }
                }
            },
            None => {
            }
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
                // panic!("should not happend index: {}, length: {}", self.index, self.length);
                self.index = 0;
                return KeywordStatus::Error;
            }
        }
        if self.index == self.length {
            match &is_end_cb {
                Some(_) => {
                },
                None => {
                    self.index = 0;
                    return KeywordStatus::Matched;
                }
            }
        }
        /*
        if self.index == self.length {
            match &is_end_cb {
                Some(cb) => {
                    if (cb)() {
                        /*
                         * 结束 => 匹配
                         * */
                        self.index = 0;
                        return KeywordStatus::Matched;
                    } else {
                        /*
                         * 没有结束 => 继续
                         * */
                        return KeywordStatus::Continue;
                    }
                },
                None => {
                    self.index = 0;
                    return KeywordStatus::Matched;
                }
            }
        }
        */
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
    // #[ignore]
    fn keyword_matched_test() {
        let stmt = "func function func fn".as_bytes();
        let mut func = Keyword::new("func".as_bytes());
        let mut function = Keyword::new("function".as_bytes());
        for c in stmt {
            match func.matched(c, Some(|c| {
                if *c as char == ' ' {
                    return true;
                }
                false
            })) {
                KeywordStatus::Matched => {
                    println!("metched func");
                },
                _ => {
                }
            }
            match function.matched(c, None) {
                KeywordStatus::Matched => {
                    println!("matched function");
                },
                _ => {
                }
            }
        }
    }
}
