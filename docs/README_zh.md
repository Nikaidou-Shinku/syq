## Sucking YQTB

[English](../README.md) | 中文

哈哈，疫情填报系统，你🐴4️⃣啦！

### 使用方法

如果你已经返校，就把 `at_school.example.toml` 文件重命名成 `config.toml`；否则把 `other.example.toml` 文件重命名成 `config.toml`。

然后打开 `config.toml`，照猫画虎把你的信息填上。如果你不希望把你的用户名密码写在本地文件里，也可以不写，把 `[basic]` 及以下两行删掉，或是只删 `username` 和 `password` 中的一个，都可以，随便你。

接下来打开终端执行：

```bash
./syq
```

就会进行单次填报。

如果你希望一直填报直到成功，可以执行：

```bash
./syq -r
```

如果你把配置文件里的用户名密码删了，需要在使用时补上：

```bash
./syq -u <username> -p <password>
```

注意把用户名密码分别替换 `<username>` 和 `<password>`，记得别留尖括号。

更多信息可以执行：

```bash
./syq -h
```
