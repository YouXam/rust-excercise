# rust-excercise

学习 rust 时候写的一些小练习，大部分是跟着教程写的，自己修改的部分在下文有注明。

## [rust-by-example](./rust-by-example/)

参见 <https://rustwiki.org/zh-CN/rust-by-example/index.html>。

## [mgrep](./mgrep/)

参考 <https://kaisery.github.io/trpl-zh-cn/ch12-00-an-io-project.html>。

1. 支持从标准输入读取；
2. 使用 kmp 算法优化了效率；
3. 加上了彩色输出。

## [web-server](./web_server/)

参考 <https://kaisery.github.io/trpl-zh-cn/ch20-00-final-project-a-web-server.html>。

1. 当接收到 SIGINT 信号时[优雅停机和处理](https://kaisery.github.io/trpl-zh-cn/ch20-03-graceful-shutdown-and-cleanup.html)。

运行服务器之后使用 `test.py` 可以测试线程池和停机清理(在运行的时候按`Ctrl+C`)。

## [async-web-server](./async_web_server/)

参考 <https://course.rs/advance/async/web-server.html>。

1. 把 for_each_concurrent 改成了 while 循环，实际测试这样效率高一点。

## [async-web-server-tokio](./async_web_server_tokio/)

上面的 async-web-server 用的是 async-std，这个用的是 tokio，大概快了 30%。