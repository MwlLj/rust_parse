use std::collections::HashMap;

pub fn parse<'a>(url: &'a str) -> (&'a str, Option<HashMap<String, String>>) {
    let index = match url.find('?') {
        Some(i) => i,
        None => {
            // println!("find ? error");
            return (url, None);
        }
    };
    let mut params: HashMap<String, String> = HashMap::new();
    let mut mode: u8 = 0;
    let mut key = String::new();
    let mut value = String::new();
    let len = url.len() - index - 1;
    let mut i = 0;
    for ch in url[(index+1)..].chars() {
        if ch == '&' {
            params.insert(key.clone(), value.clone());
            key.clear();
            value.clear();
            mode = 0;
        } else if ch == '=' {
            mode = 1;
        }
        if ch == '&' || ch == '=' {
            i += 1;
            continue;
        }
        if mode == 0 {
            key.push(ch);
        } else if mode == 1 {
            value.push(ch);
        }
        if i == len - 1 {
            params.insert(key.clone(), value.clone());
        }
        i += 1;
    }
    (&url[0..index], Some(params))
}

#[test]
fn parseTest() {
    let (url, params) = parse("/index?username=liujun&userpwd=123456&age=20");
    println!("url: {}, params: {:?}", url, params);
}
