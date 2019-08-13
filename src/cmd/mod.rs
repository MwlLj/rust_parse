use std::env;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct CCmd {
    help: String,
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
            if arg == self.help {
                self.printHelp();
                self.exit();
            }
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
    fn printHelp(&self) {
        println!("help:");
        for (key, value) in self.keys.iter() {
            println!("\t{}: {}", key, *value.borrow());
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
