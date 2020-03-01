use std::collections::HashMap;

struct Object(HashMap<String, Value>);

impl Object {
    fn insert(&mut self, k: String, v: Value) {
        self.0.insert(k, v);
    }
    fn new() -> Self {
        Self(HashMap::new())
    }
}

struct Array(Vec<Value>);

enum Value {
    None,
    Str(String),
    Int32(i32),
    Float32(f32),
    Object(Object),
    Array(Array)
}

enum Token {
    Symbol(char),
    QuotaVal(String),
}

enum TokenMode {
    Normal,
    DoubleQuota
}

enum ParseMode {
    Normal,
    Big,
    Mid,
    Quota
}

enum InnerMode {
    Normal,
    FindQuota,
    Quota,
    FindColon,
    Colon,
    FindComma,
    ValueQuota,
    Comma
}

struct Inner {
    mode: InnerMode,
    word: String,
    key_tmp: String,
    value: Value
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            mode: InnerMode::Normal,
            word: String::new(),
            key_tmp: String::new(),
            value: Value::None
        }
    }
}

impl Inner {
    fn clear(&mut self) {
        self.mode = InnerMode::Normal;
        self.word.clear();
        self.key_tmp.clear();
        self.value = Value::None;
    }
}

pub struct Json {
}

impl Json {
    pub fn from_str(&self, s: &str) {
        let chars = s.chars();
        let mut parse_mode = ParseMode::Normal;
        let mut inner_data = Inner::default();
        let mut value = Value::None;
        for c in chars {
            self.object_parse(c, &mut parse_mode, &mut inner_data, &mut value);
        }
    }
}

impl Json {
    fn object_parse(&self, c: char, parse_mode: &mut ParseMode, inner_data: &mut Inner, value: &mut Value) {
        match parse_mode {
            ParseMode::Normal => {
                match c {
                    '{' => {
                        inner_data.value = Value::Object(Object::new());
                        *parse_mode = ParseMode::Big;
                        inner_data.mode = InnerMode::FindQuota;
                    },
                    '[' => {
                    },
                    _ => {
                    }
                }
            },
            ParseMode::Big => {
                if c == '}' {
                }
                match &inner_data.mode {
                    InnerMode::FindQuota => {
                        if c == '"' {
                            inner_data.mode = InnerMode::Quota;
                        } else {
                        }
                    },
                    InnerMode::Quota => {
                        if c == '"' {
                            inner_data.key_tmp = inner_data.word.clone();
                            inner_data.mode = InnerMode::FindColon;
                            inner_data.word.clear();
                        } else {
                            inner_data.word.push(c);
                        }
                    },
                    InnerMode::FindColon => {
                        if c == ':' {
                            inner_data.mode = InnerMode::Colon;
                        } else {
                        }
                    },
                    InnerMode::Colon => {
                        if c == ',' {
                        } else if c == '"' {
                            inner_data.mode = InnerMode::ValueQuota;
                        } else if c == '{' || c == '[' {
                        } else {
                        }
                    },
                    InnerMode::ValueQuota => {
                        if c == '"' {
                            inner_data.mode = InnerMode::Colon;
                            inner_data.word.clear();
                        } else {
                            inner_data.word.push(c);
                        }
                    },
                    _ => {
                    }
                }
            },
            _ => {
            }
        }
    }
}

/*
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
*/
