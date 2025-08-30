# 解释如下代码
```shell
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("Hello, crossteam!"),
        ResetColor,
    )?;
```
这行代码的意思是：使用 `crossterm` 库在终端输出带有蓝色前景色的 "Hello, crossteam!"，然后重置颜色。  
具体步骤如下：

1. `SetForegroundColor(Color::Blue)`：设置终端文字颜色为蓝色。
2. `Print("Hello, crossteam!")`：打印字符串到终端。
3. `ResetColor`：恢复终端默认颜色。

`execute!` 宏会依次执行这些操作，`stdout()` 表示输出到标准输出（终端）。

# enable_raw_mode()?; 是什么意思
`enable_raw_mode()?;` 的作用是：让终端进入“原始模式”，关闭输入的标准处理（比如回显、行缓冲等），这样可以直接读取用户输入的每个按键。  
常用于需要捕获键盘事件的命令行程序。  
`?` 表示如果出错会自动返回错误。

# 解释代码
```rust
    let Some(event)=event.as_key_press_event() else {
                continue;
            };
```
这段代码的意思是：尝试将 event 转换为按键事件（KeyEvent），
如果转换失败（即不是按键事件），就跳过本次循环，继续等待下一个事件。
# 解释代码
```rust
        if (event.code == KeyCode::Char('c') && event.modifiers == KeyModifiers::CONTROL) {
            break;
        }
```
ctrl+c退出
# 解释代码
```rust
queue!(
        write,
        EnterAlternateScreen,// 进入可选择的屏幕
        Hide // 隐藏鼠标
    ) 
```
这行代码的作用是将多个终端操作（进入备用屏幕、隐藏光标）排队，准备写入到 write（通常是标准输出或标准错误）。
queue! 宏不会立即执行这些操作，而是把它们写入缓冲区，等你调用 flush() 时才真正生效。
EnterAlternateScreen 让终端进入备用屏幕，Hide 隐藏光标。
这样可以实现更好的终端界面效果。