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
    pub fn matched(c: u8) -> KeywordStatus {
    }

    pub fn new(src: &'a [u8]) -> Self {
        Self{
            src: src,
            index: 0,
            length: src.len()
        }
    }
}
