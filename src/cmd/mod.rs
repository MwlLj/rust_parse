use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

struct CItem(Rc<RefCell<String>>, String, bool);

pub struct CCmd {
    help: String,
    keys: HashMap<String, CItem>
}

impl CCmd {
    pub fn register(&mut self, key: &str, default: &str) -> Rc<RefCell<String>> {
        self.register_with_desc(key, default, "")
    }

    pub fn register_with_desc(&mut self, key: &str, default: &str, desc: &str) -> Rc<RefCell<String>> {
        let r = Rc::new(RefCell::new(default.to_string()));
        self.keys.insert(key.to_string(), CItem(r.clone(), desc.to_string(), false));
        r.clone()
    }

    pub fn has(&self, key: &str) -> bool {
        let v = match self.keys.get(key) {
            Some(v) => v,
            None => {
                return false;
            }
        };
        v.2
    }

    pub fn parse(&mut self) {
        let args = env::args();
        let argsLen = args.len();
        let mut isFind = false;
        let mut lastKey = "".to_string();
        for (index, arg) in args.enumerate() {
            if arg == self.help {
                self.printHelp();
                self.exit();
            }
            match self.keys.get(&arg) {
                Some(field) => {
                    isFind = true;
                    lastKey = arg;
                    if let Some(r) = self.keys.get_mut(&lastKey) {
                        r.2 = true;
                    };
                },
                None => {
                    if isFind == true {
                        if let Some(r) = self.keys.get_mut(&lastKey) {
                            *(*r.0/*get HashMap mut value*/).borrow_mut() = arg;
                        }
                    }
                    isFind = false;
                }
            }
        }
    }
}

impl CCmd {
    fn printHelp(&self) {
        println!("help:");
        for (key, value) in self.keys.iter() {
            println!("\t{}\n\t\tdefault: {}\n\t\tdesc: {}", key, *value.0.borrow(), &value.1);
        }
    }

    fn exit(&self) {
        if cfg!(target_os="windows") {
            std::process::exit(0);
        } else {
            std::process::exit(0);
        }
    }
}

impl CCmd {
    pub fn new() -> CCmd {
        CCmd{
            help: "--help".to_string(),
            keys: HashMap::new()
        }
    }
}
