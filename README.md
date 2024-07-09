# Append ZIP

一天一个小知识之 ZIP 的末端识别。

今天早晨看到了这篇帖子：[将资源嵌入到可执行文件中并保持目录结构 - V2EX](https://v2ex.com/t/1055626)，众所周知 Go 原生支持多种文件嵌入可执行文件的方式，这是一个非常实用的小特性，那么对于不支持这一特性的语言应该如何实现呢？这篇帖子给了我们一个很好的思路，即将 ZIP 文件拼接到编译好的可执行文件的尾部。

## 使用方法

```shell-session
$ append-zip <action> <directory>
```

- `<action>`：要执行的操作（`box` 或 `unbox`）。
- `<directory>`：要压缩或提取的目录。

## Todo

- [ ] 增加 VFS。
- [ ] 使用宏扩展功能？

## 许可证

该项目采用 MIT 许可证。详情请参阅 [LICENSE](LICENSE) 文件。
