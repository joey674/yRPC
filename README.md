# rpc
用户自定义调用方法设定为同步方法；
用户自定义调用方法将会默认为计算密集型，并使用并行阻塞线程执行


# 使用宏
装cargo-expand库
cargo expand --bin callee > expanded_code.rs> expanded_code.rs 可以把宏拓展的代码编译出来 但是记得把测试例子写成bin放进toml中

# TODO
- 拓展用户设定方法的类型 可以动态分配是否为计算密集或者是io密集来使用任务分配方式；

