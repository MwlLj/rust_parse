use std::collections::HashMap;
use std::any::{TypeId, Any};

#[derive(Debug, Clone)]
pub struct Object(HashMap<String, Value>);

impl Object {
    fn insert(&mut self, k: String, v: Value) {
        self.0.insert(k, v);
    }
    fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Debug, Clone)]
pub struct Array(Vec<Value>);

impl Array {
    fn push_back(&mut self, v: Value) {
        self.0.push(v);
    }
    fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    None,
    Str(String),
    Int64(i64),
    Float64(f64),
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
    FindMidNext,
    FindQuota,
    Quota,
    FindColon,
    Colon,
    FindComma,
    ValueQuota,
    Comma
}

enum Code {
    End,
    Continue,
    Error
}

enum ValueParseMode {
    Integer,
    Decimal
}

struct Inner {
    mode: InnerMode,
    word: String,
    key_tmp: String,
    value_tmp: Value,
    value: Value
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            mode: InnerMode::Normal,
            word: String::new(),
            key_tmp: String::new(),
            value_tmp: Value::None,
            value: Value::None
        }
    }
}

impl Inner {
    fn clear(&mut self) {
        self.mode = InnerMode::Normal;
        self.word.clear();
        self.key_tmp.clear();
        self.value_tmp = Value::None;
        self.value = Value::None;
    }
}

pub struct Json {
}

impl Json {
    pub fn from_str(&self, s: &str) -> Value {
        let mut chars = s.chars();
        let mut parse_mode = ParseMode::Normal;
        let mut inner_data = Inner::default();
        let mut value = Value::None;
        self.object(&mut chars, &mut parse_mode, &mut inner_data, &mut value);
        value
    }
}

impl Json {
    fn object(&self, chars: &mut std::str::Chars, parse_mode: &mut ParseMode, inner_data: &mut Inner, value: &mut Value) -> Result<(), &str> {
        // let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match self.object_parse(chars, c, parse_mode, inner_data, value) {
                Code::End => {
                    break;
                },
                Code::Continue => {
                },
                Code::Error => {
                    return Err("object parse error");
                }
            }
        }
        Ok(())
    }

    fn object_parse(&self, chars: &mut std::str::Chars, c: char, parse_mode: &mut ParseMode, inner_data: &mut Inner, value: &mut Value) -> Code {
        match parse_mode {
            ParseMode::Normal => {
                match c {
                    '{' => {
                        inner_data.value = Value::Object(Object::new());
                        *parse_mode = ParseMode::Big;
                        inner_data.mode = InnerMode::FindQuota;
                    },
                    '[' => {
                        inner_data.value = Value::Array(Array::new());
                        *parse_mode = ParseMode::Mid;
                        inner_data.mode = InnerMode::FindMidNext;
                    },
                    _ => {
                    }
                }
            },
            ParseMode::Big => {
                if c == '}' {
                    match &mut inner_data.value {
                        Value::Object(obj) => {
                            let mut v = Value::None;
                            match &inner_data.value_tmp {
                                Value::Str(s) => {
                                    v = Value::Str(s.to_string());
                                },
                                Value::Object(obj) => {
                                    v = Value::Object(obj.clone());
                                },
                                Value::Array(arr) => {
                                    v = Value::Array(arr.clone());
                                },
                                // Value::None => {
                                // },
                                _ => {
                                    /*
                                    ** 数值
                                    */
                                    v = match self.numstr_to_value(&inner_data.word) {
                                        Ok(v) => v,
                                        Err(err) => {
                                            println!("str to value error, err: {}", err);
                                            return Code::Error;
                                        }
                                    };
                                }
                            }
                            obj.insert(inner_data.key_tmp.clone(), v);
                            *value = inner_data.value.clone();
                        },
                        _ => {
                        }
                    }
                    *parse_mode = ParseMode::Normal;
                    inner_data.clear();
                    return Code::End;
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
                            match &mut inner_data.value {
                                Value::Object(obj) => {
                                    let mut v = Value::None;
                                    match &inner_data.value_tmp {
                                        Value::Str(s) => {
                                            v = Value::Str(s.to_string());
                                        },
                                        Value::Object(obj) => {
                                            v = Value::Object(obj.clone());
                                        },
                                        Value::Array(arr) => {
                                            v = Value::Array(arr.clone());
                                        },
                                        // Value::None => {
                                        // },
                                        _ => {
                                            /*
                                            ** 数值
                                            */
                                            v = match self.numstr_to_value(&inner_data.word) {
                                                Ok(v) => v,
                                                Err(err) => {
                                                    println!("str to value error, err: {}", err);
                                                    return Code::Error;
                                                }
                                            };
                                        }
                                    }
                                    obj.insert(inner_data.key_tmp.clone(), v);
                                },
                                _ => {
                                }
                            }
                            inner_data.word.clear();
                            inner_data.mode = InnerMode::FindQuota;
                            inner_data.value_tmp = Value::None;
                        } else if c == '"' {
                            inner_data.mode = InnerMode::ValueQuota;
                        } else if c == '{' {
                            let mut parse_mode_sub = ParseMode::Big;
                            let mut inner_data_sub = Inner::default();
                            let mut value_sub = Value::None;
                            inner_data_sub.mode = InnerMode::FindQuota;
                            inner_data_sub.value = Value::Object(Object::new());
                            /*
                            ** 遇到嵌套的 object
                            */
                            self.object(chars, &mut parse_mode_sub, &mut inner_data_sub, &mut value_sub);
                            inner_data.value_tmp = value_sub;
                        } else if c == '[' {
                            let mut parse_mode_sub = ParseMode::Mid;
                            let mut inner_data_sub = Inner::default();
                            let mut value_sub = Value::None;
                            inner_data_sub.mode = InnerMode::FindMidNext;
                            inner_data_sub.value = Value::Array(Array::new());
                            /*
                            ** [{}]
                            */
                            self.object(chars, &mut parse_mode_sub, &mut inner_data_sub, &mut value_sub);
                            inner_data.value_tmp = value_sub;
                        } else {
                            if !c.is_ascii_whitespace() {
                                inner_data.word.push(c);
                            }
                        }
                    },
                    InnerMode::ValueQuota => {
                        if c == '"' {
                            inner_data.value_tmp = Value::Str(inner_data.word.clone());
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
            ParseMode::Mid => {
                if c == ']' || c == ',' {
                    match &mut inner_data.value {
                        Value::Array(arr) => {
                            let mut v = Value::None;
                            match &inner_data.value_tmp {
                                Value::Str(s) => {
                                    v = Value::Str(s.to_string());
                                },
                                Value::Object(obj) => {
                                    v = Value::Object(obj.clone());
                                },
                                Value::Array(arr) => {
                                    v = Value::Array(arr.clone());
                                },
                                _ => {
                                    /*
                                    ** 数值
                                    */
                                    v = match self.numstr_to_value(&inner_data.word) {
                                        Ok(v) => v,
                                        Err(err) => {
                                            println!("str to value error, err: {}", err);
                                            return Code::Error;
                                        }
                                    };
                                }
                            }
                            arr.push_back(inner_data.value_tmp.clone());
                            if c == ']' {
                               *value = inner_data.value.clone();
                            }
                        },
                        _ => {
                            /*
                            ** 这里不会执行到
                            */
                        }
                    }
                    if c == ',' {
                        inner_data.word.clear();
                        inner_data.mode = InnerMode::FindMidNext;
                        inner_data.value_tmp = Value::None;
                        return Code::Continue;
                    }
                    if c == ']' {
                        inner_data.clear();
                        *parse_mode = ParseMode::Normal;
                        return Code::End;
                    }
                }
                match inner_data.mode {
                    InnerMode::FindMidNext => {
                        /*
                        ** [ 后可以跟 { 或者 " 或者 无
                        */
                        if c == '"' {
                            /*
                            ** [""]
                            */
                            inner_data.mode = InnerMode::Quota;
                        } else if c == '{' {
                            let mut parse_mode_sub = ParseMode::Big;
                            let mut inner_data_sub = Inner::default();
                            let mut value_sub = Value::None;
                            inner_data_sub.mode = InnerMode::FindQuota;
                            inner_data_sub.value = Value::Object(Object::new());
                            /*
                            ** [{}]
                            */
                            self.object(chars, &mut parse_mode_sub, &mut inner_data_sub, &mut value_sub);
                            inner_data.value_tmp = value_sub;
                        } else if c == '[' {
                            /*
                            ** [[]]
                            */
                            let mut parse_mode_sub = ParseMode::Mid;
                            let mut inner_data_sub = Inner::default();
                            let mut value_sub = Value::None;
                            inner_data_sub.mode = InnerMode::FindMidNext;
                            inner_data_sub.value = Value::Array(Array::new());
                            /*
                            ** [{}]
                            */
                            self.object(chars, &mut parse_mode_sub, &mut inner_data_sub, &mut value_sub);
                            inner_data.value_tmp = value_sub;
                        } else {
                            if !c.is_ascii_whitespace() {
                                /*
                                ** 数值 => [1, 2, 3]
                                */
                                inner_data.word.push(c);
                            }
                        }
                    },
                    InnerMode::Quota => {
                        if c == '"' {
                            inner_data.value_tmp = Value::Str(inner_data.word.clone());
                            inner_data.mode = InnerMode::FindMidNext;
                            inner_data.word.clear();
                        } else {
                            inner_data.word.push(c);
                        }
                    },
                    _ => {
                    }
                }
            }
            _ => {
            }
        }
        return Code::Continue;
    }

    fn numstr_to_value(&self, s: &str) -> Result<Value, &str> {
        let mut value = Value::None;
        let chars = s.chars();
        let mut mode = ValueParseMode::Integer;
        let mut integer: u64 = 0;
        let mut decimal: f64 = 0.0;
        let mut decimal_rate: f64 = 0.1;
        let mut p_n_symbol: i64 = 1;
        for c in chars {
            if c == '-' {
                p_n_symbol = -1;
                continue;
            }
            match mode {
                ValueParseMode::Integer => {
                    if c == '.' {
                        mode = ValueParseMode::Decimal;
                    } else {
                        if c >= '0' && c <= '9' {
                            integer *= 10;
                            integer += (c as u8 - 48) as u64;
                        }
                    }
                },
                ValueParseMode::Decimal => {
                    if c >= '0' && c <= '9' {
                        decimal += (c as u8 - 48) as f64 * decimal_rate;
                        decimal_rate /= 10.0;
                    }
                }
            }
        }
        match mode {
            ValueParseMode::Integer => {
                value = Value::Int64(p_n_symbol * integer as i64);
            },
            ValueParseMode::Decimal => {
                value = Value::Float64(p_n_symbol as f64 * (integer as f64 + decimal));
            }
        }
        Ok(value)
    }
}

impl Json {
    pub fn new() -> Json {
        let obj = Json{};
        obj
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn from_str_object_test() {
        let json_parser = Json::new();
        let value = json_parser.from_str(r#"
        {
            "name": "Jake",
            "age": 20,
            "obj1": {
                "f1": "v1",
                "f2": "v2"
            }
        }
            "#);
        println!("{:?}", value);
    }

    #[test]
    #[ignore]
    fn from_str_array_test() {
        let json_parser = Json::new();
        let value = json_parser.from_str(r#"
        [
            {
                "name": "Jake",
                "age": 20,
                "obj1": {
                    "f1": "v1",
                    "f2": "v2"
                },
                "arr1": ["1", "2", "3"],
                "arr2": [
                    {
                        "f3": "v3",
                        "f4": "v4"
                    }
                ]
            },
            {
                "name": "Jake",
                "age": 20,
                "obj1": {
                    "f1": "v1",
                    "f2": "v2"
                }
            }
        ]
            "#);
        println!("{:?}", value);
    }

    #[test]
    #[ignore]
    fn numstr_to_value_test() {
        let json_parser = Json::new();
        let value = json_parser.numstr_to_value("-123789.456");
        println!("{:?}", value);
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
