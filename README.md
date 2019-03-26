# rust_parse
rust 的解析库

## 说明
模仿 golang 的 flag 包中 命令行的解析方式


## 使用方式
1. 创建实例
    let mut cmdHandler = CCmd::new();
2. 设置默认值
    let ip = cmdHandler.register("-ip", "localhost");
    -ip 就是命令行参数名
    localhost 就是 -ip 的默认值
3. 调用解析
    cmdHandler.parse();
4. 获取结果
    let ip = ip.borrow();

