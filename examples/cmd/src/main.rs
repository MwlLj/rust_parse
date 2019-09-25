extern crate rust_parse;

use std::rc::Rc;

use rust_parse::cmd::CCmd;

fn main() {
    let mut cmdHandler = CCmd::new();
    let ip = cmdHandler.register("-ip", "localhost");
    let port = cmdHandler.register("-port", "50000");
    let proto = cmdHandler.register_with_desc("-proto", "http", "interface proto type");
    let stopall = cmdHandler.register("stopall", "");
    cmdHandler.parse();

    let ip = ip.borrow();
    let port = port.borrow();
    let stopall = stopall.borrow();

    if cmdHandler.has("stopall") {
        println!("exist stopall");
    }
    if cmdHandler.has("-ip") {
        println!("exist -ip");
    }

    println!("ip: {}, port: {}", ip, port);
}
