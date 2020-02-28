enum Value {
    Str(String),
    Int32(i32),
    Float32(f32)
}

enum Token {
    Symbol(char),
    QuotaVal(String),
}

enum TokenMode {
    Normal,
    DoubleQuota
}

pub struct Json {
}

impl Json {
    pub fn from_str(&self, s: &str) {
    }
}

impl Json {
    fn token_extract(&self, s: &str) {
        let mut tokens: Vec<Token> = Vec::new();
    }
    /*
    ** 提取 token
    */
    fn token_extract2(&self, s: &str) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut token_mode = TokenMode::Normal;
        let mut word = String::new();
        for c in s.chars() {
            match token_mode {
                TokenMode::Normal => {
                    if c.is_ascii_whitespace() {
                        continue;
                    }
                    match c {
                        '"' => {
                            token_mode = TokenMode::DoubleQuota;
                        },
                        '{'|'}'|'['|']'|','|':' => {
                            let token = Token::Symbol(c);
                            tokens.push(token);
                        },
                        _ => {
                        }
                    }
                },
                TokenMode::DoubleQuota => {
                    match c {
                        '"' => {
                            tokens.push(Token::QuotaVal(word.clone()));
                            token_mode = TokenMode::Normal;
                            word.clear();
                        },
                        _ => {
                            word.push_back(c);
                        }
                    }
                }
            }
        }
    }
}
