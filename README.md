# oops

小玩意,监听键盘全局的按压事件,然后统计每个键敲击了多少次.

### 如何使用:

#### 安装
```shell
git clone git@github.com:ltoddy/oops.git
cd oops
cargo install --path .
mkdir ~/.oops
```

#### 启动

```shell
oops listen
```

运行此命了之后,在`~/.oops`目录下面会出现`oops.socket`文件.

#### 查看键盘敲击情况

```shell
oops status
```


#### TODO

[] 添加守护进程,后台执行
[] server自动重启
[] 美化输出
