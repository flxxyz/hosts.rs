# hosts.rs

一个可以订阅并快速合并生成hosts的工具

## 起步

```bash
# output: hosts.rs 0.0.1
cargo run -- -V
```

### 基础指令

> 更多选项查看对应指令的帮助信息 hosts \<subcommend\> -help

```bash
# 查看帮助信息
hosts help

# 添加一个订阅的hosts链接
# alias: a
hosts add https://raw.githubusercontent.com/flxxyz/hosts.rs/master/hosts

# 查看添加的链接
# alias: ls, l
hosts list

# 拉取合并所有订阅的hosts
# alias: f
hosts fetch
```
