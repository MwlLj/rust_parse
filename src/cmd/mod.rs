use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct CCmd {
    keys: HashMap<String, Rc<RefCell<String>>>
}

impl CCmd {
    pub fn register(&mut self, key: &str, default: &str) -> Rc<RefCell<String>> {
        let r = Rc::new(RefCell::new(default.to_string()));
        self.keys.insert(key.to_string(), r.clone());
        r.clone()
    }
    pub fn parse(&mut self) {
        let args = env::args();
        let argsLen = args.len();
        let mut isFind = false;
        let mut lastKey = "".to_string();
        for (index, arg) in args.enumerate() {
            match self.keys.get(&arg) {
                Some(field) => {
                    isFind = true;
                    lastKey = arg;
                },
                None => {
                    if isFind == true {
                        if let Some(r) = self.keys.get_mut(&lastKey) {
                            *(*r/*get HashMap mut value*/).borrow_mut() = arg;
                        }
                    }
                    isFind = false;
                }
            }
        }
    }
}

impl CCmd {
    pub fn new() -> CCmd {
        CCmd{
            keys: HashMap::new()
        }
    }
}
