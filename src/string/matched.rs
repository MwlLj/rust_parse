/*
 * 缓存输入, 并每次进行是否和输入匹配
 * */
/*
 * 匹配到了长度, 但是是否结束
 * */
pub type IsEndCB = fn(c: &u8) -> bool;

pub struct Keyword<'a, MatchedF: FnMut()> {
    src: &'a [u8],
    index: usize,
    length: usize,
    is_end_cb: IsEndCB,
    matched_f: MatchedF
}

pub enum KeywordStatus {
    Error,
    Continue,
    Matched
}


impl<'a, MatchedF: FnMut()> Keyword<'a, MatchedF> {
    pub fn matched(&mut self, c: &u8) {
        if (self.is_end_cb)(c) {
            if self.index == self.length {
                self.index = 0;
                (self.matched_f)();
            }
        }
        if self.index >= self.length {
            self.index = 0;
            return;
        }
        match self.src.get(self.index) {
            Some(v) => {
                if v == c {
                    self.index += 1;
                } else {
                    self.index = 0;
                    return;
                }
            },
            None => {
                panic!("should not happend");
            }
        }
    }

    pub fn new(src: &'a [u8], is_end_cb: IsEndCB,
            matched_f: MatchedF) -> Self {
        if src.len() == 0 {
            panic!("src must be more than zero");
        }
        Self{
            src: src,
            index: 0,
            length: src.len(),
            is_end_cb: is_end_cb,
            matched_f: matched_f
        }
    }
}

impl<'a, MatchedF: FnMut()> std::ops::Drop for Keyword<'a, MatchedF> {
    fn drop(&mut self) {
        if self.index == self.length {
            (self.matched_f)();
        }
    }
}

mod test {
    use super::*;

    fn keyword_matched_end_cb(c: &u8) -> bool {
        if *c as char == ' ' {
            return true;
        }
        false
    }

    #[test]
    // #[ignore]
    fn keyword_matched_test() {
        let stmt = "funcx fun func function func".as_bytes();
        let mut func = Keyword::new("func".as_bytes()
            , keyword_matched_end_cb
            , || {
            println!("match func");
        });
        let mut function = Keyword::new("function".as_bytes()
            , keyword_matched_end_cb
            , || {
            println!("match function");
        });
        for c in stmt {
            func.matched(c);
            function.matched(c);
        }
    }
}
