use std::net::TcpStream;
use std::io;
use std::io::prelude::*;

pub enum Code {
    End,
    Continue,
    Error,
    Failed
}

pub struct CStreamBlockParse {
    reader: Box<dyn io::Read + 'static>
}

impl CStreamBlockParse {
    pub fn line<F, T>(&mut self, startLen: u64, t: &mut T, f: &mut F) -> Result<Code, &str>
        where F: FnMut(u64, Vec<u8>, &mut T) -> (Code, u64) {
        let mut len = startLen;
        let mut index = 0;
        let mut total = 0;
        loop {
            let mut buf = Vec::new();
            if let Ok(size) = self.reader.by_ref().take(len as u64).read_to_end(&mut buf) {
                if len > 0 && size == 0 {
                    return Err("read end");
                }
                let (b, l) = f(index, buf, t);
                match b {
                    Code::End => {
                        // one package read end
                        break;
                    },
                    Code::Continue => {
                        // nothing, continue
                    },
                    Code::Error => {
                        return Ok(Code::Error);
                    },
                    Code::Failed => {
                        return Err("f return error");
                    }
                }
                len = l;
                index += 1;
                total += size;
            } else {
                println!("read error");
                return Err("read error");
            }
        }
        if total == 0 {
            return Err("read end");
        }
        Ok(Code::End)
    }

    pub fn lines<F, T, L>(&mut self, startLen: u64, t: &mut T, f: &mut F, lf: &mut L) -> Result<(), &str>
        where F: FnMut(u64, Vec<u8>, &mut T) -> (Code, u64), L: FnMut(&mut T, Code) -> bool {
        while let Ok(code) = self.line(startLen, t, f) {
            if !lf(t, code) {
                return Ok(())
            }
        }
        Ok(())
    }

    pub fn clearReadBuffer(&mut self, offset: u64) -> Result<(), &str> {
        loop {
            let mut buf = Vec::new();
            if let Ok(size) = self.reader.by_ref().take(offset).read_to_end(&mut buf) {
                if size == 0 {
                    break;
                }
            } else {
                return Err("read to end error");
            }
        }
        Ok(())
    }
}

impl CStreamBlockParse {
    pub fn new<T>(t: T) -> CStreamBlockParse
        where T: io::Read + 'static {
        CStreamBlockParse{
            reader: Box::new(t)
        }
    }
}

#[test]
#[ignore]
fn streamBlockParseTest() {
    /*
    r.lines(32, &mut req, &mut |index: u64, buf: Vec<u8>, request: &mut CRequest| -> (bool, u64) {
        decode_request!(index, buf, request);
    }, |request: &CRequest| -> bool {
    });
    */
}

