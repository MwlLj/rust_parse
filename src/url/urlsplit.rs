#[derive(Default, Debug, Clone)]                
pub struct Param {
    pub key: String,
    pub value: String
}

impl Param {
    fn clear(&mut self) {
        self.key.clear();
        self.value.clear();
    }
}

#[derive(Default, Debug)]
pub struct Url {
    pub proto: String,
    pub addr: String,
    pub path: String,
    pub params: Vec<Param>
}

enum mode {
    proto,
    addr,
    path,
    param
}

enum kvmode {
    normal,
    key,
    value
}

fn parse<'a>(url: &'a str) -> Result<Url, &'a str> {
    let mut u = Url::default();
    let mut m = mode::proto;
    let mut kvm = kvmode::normal;
    let mut ib = 0;
    let mut isKvStart = true;
    let mut param = Param::default();
    for c in url.chars() {
        match m {
            mode::proto => {
                if ib == 2 && c == '/' {
                    ib = 0;
                    m = mode::addr;
                    continue;
                }
                if c != ':' && c != '/' {
                    u.proto.push(c);
                } else {
                    ib += 1;
                }
            },
            mode::addr => {
                if c == '/' {
                    u.path.push('/');
                    m = mode::path;
                    continue;
                } else {
                    u.addr.push(c);
                }
            },
            mode::path => {
                if c == '?' {
                    m =  mode::param;
                    kvm = kvmode::key;
                    continue;
                } else {
                    u.path.push(c);
                }
            },
            mode::param => {
                match kvm {
                    kvmode::normal => {
                    },
                    kvmode::key => {
                        if c == '=' {
                            kvm = kvmode::value;
                        } else {
                            param.key.push(c);
                        }
                    },
                    kvmode::value => {
                        if c == '&' {
                            u.params.push(param.clone());
                            param.clear();
                            kvm = kvmode::key;
                        } else {
                            param.value.push(c);
                        }
                    }
                }
            }
        }
    }
    match kvm {
        kvmode::normal => {
        },
        _ => {
            u.params.push(param);
        }
    }
    Ok(u)
}

mod test {
    use super::*;
    #[test]
    fn parseTest() {
        let url = "http://127.0.0.1:50000/index?user=Jake&pwd=123456";
        match parse(url) {
            Ok(u) => {
                println!("{:?}", u);
            },
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
