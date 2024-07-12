# 大作业：Wordle（MoonBit 版）

请注意：此文档仅描述使用 MoonBit
完成作业的必要步骤，所有其他作业要求均以课程文档为准。

## 环境准备

注意：MoonBit 语言正在活跃开发中。为了批改方便，**请勿自行升级版本**！

### 安装 MoonBit 工具链

通过如下命令安装**指定版本**的 MoonBit 工具链：

```bash
curl -fsSL https://cli.moonbitlang.cn/install/unix.sh | bash -s 0.1.20240703+9f66d0525
export PATH="$HOME/.moon/bin:$PATH"
```

安装后可检查版本详细信息，确认与下面一致：

```bash
$ moon version --all
moon 0.1.20240703 (9f69ed8 2024-07-03) ~/.moon/bin/moon
moonc v0.1.20240703+9f66d0525 ~/.moon/bin/moonc
moonrun 0.1.20240703 (80a1a15 2024-07-03) ~/.moon/bin/moonrun
```

上述命令自动安装的标准库 `moonbitlang/core` 版本为
<https://github.com/moonbitlang/core/tree/8f3bfab>。

### 安装 VS Code 插件

为方便开发，MoonBit 提供了 VS Code 插件，名为 "MoonBit
Language"，插件的版本必须标准库版本一致，为 `0.1.20240703`。

安装方法：在 VS Code 左侧选择“扩展 / Extension”，搜索 "MoonBit
Language"，选择安装（注意不要选择 Nightly
版本）。如果当前的版本与上述不符，可以在插件页面中选择"卸载 /
Uninstall"右侧下拉按钮，在菜单中选择“安装另一个版本 / Install Another
Version”，在列表中选择 `0.1.20240703` 即可。

## MoonBit 开发

## 参考文档

开发过程中可能会用到的文档：

- MoonBit 语法文档：<https://www.moonbitlang.cn/docs/syntax>
- 构建系统文档：<https://www.moonbitlang.cn/docs/build-system-tutorial>
- MoonBit 标准库源码：<https://github.com/moonbitlang/core>
- MoonBit 标准库文档：<https://mooncakes.io/docs/#/moonbitlang/core/>

需要注意，上述标准库文档总是对应目前最新的 `moonbitlang/core`
仓库，可能作业使用的版本不一致。
我们推荐通过如下命令在本地构建并查看固定版本的标准库文档：

```bash
git clone https://github.com/moonbitlang/core moonbit-core
cd moonbit-core
git checkout 8f3bfab
moon doc --serve
```

## 外部接口

目前 MoonBit 的 IO 接口并未稳定，请直接使用 `./io`
目录下提供的接口，这些接口暂不考虑错误处理，如果出错将在 `moonrun`
中报错。所有可用的接口包括：

- `wordle/io`:
  - `read_line_from_stdin() -> String?`: 从 `stdin` 中读取一行，返回 `None`
    表示读到文件末尾
  - `print(obj : Show) -> Unit`: 打印一个实现了 `Show` 的对象
  - `println(obj : Show) -> Unit`: 打印一个实现了 `Show` 的对象并换行（与标准库中的 `println` 等价）
  - `write(fd : StdStream, obj : Show) -> Unit`: 向 `fd` 写入一个实现了 `Show`
    的对象，用法如 `write(StdStream::Stderr, "error message")`
  - `writeln(fd : StdStream, obj : Show) -> Unit`: 向 `fd` 写入一个实现了 `Show`
    的对象并换行
  - `flush(fd : StdStream) -> Unit`: 刷新 `fd` 的输出缓冲区
- `wordle/io/env`:
  - `get_env_var(name : String) -> String?`: 读取环境变量
  - `get_args() -> Array[String]`: 读命令行参数
- `wordle/io/fs`:
  - `read_to_string(path : String) -> String`: 读文件到字符串
  - `write_to_string(path : String, content : String) -> Unit`: 写字符串到文件
  - `exists(path : String) -> Bool`: 判断文件是否存在
- `wordle/io/rand` 提供与 Rust 中 `StdRng` 行为一致的随机数:
  - `stdrng_seed_from_u32(seed : Int) -> StdRng`
  - `stdrng_gen_range(rng : StdRng, ubound : Int) -> Int`
- `wordle/io/sys`:
  - `fn exit(code : Int)`: 以指定的退出码退出程序

若使用这些包，需要在 `./wordle/moon.pkg.json` 中增加导入（默认已经添加）：

```json
{
    "import": [
        "wordle/io",
        "wordle/io/env",
        "wordle/io/fs",
        "wordle/io/rand",
        "wordle/io/sys"
    ]
}
```

如果导入正确，当在源码中输入 `@fs.` 可以看到 IDE 的提示。

## 修改代码

使用 VS Code 打开 `wordle-mbt` 目录。注意当前 MoonBit 的 VS Code 插件必须打开以
`moon.mod.json` 为根目录的文件夹，才可正常工作。

作业目录初始结构为：

```text
.
├── README.md
├── io
│   └── ...
├── main
│   ├── main.mbt
│   └── moon.pkg.json
├── moon.mod.json
└── wordle
    ├── constant
    │   ├── builtin_words.mbt
    │   └── moon.pkg.json
    ├── game.mbt
    └── moon.pkg.json
```

其中 `./io` 与 `./main` 文件夹中的内容**不要进行修改**，游戏逻辑在 `./wordle`
文件夹中实现。 `./wordle` 可以自由修改、移动、添加新的`mbt`
文件、新的包，但是要保证使用 `./wordle/constant` 中定义的单词，以及
`./main/moon.pkg.json` 中能正确导入该包。

若 `wordle/game.mbt` 文件的内容为：

```rust
pub fn entry() -> Unit {
    println("Hello, world!")
}
```

执行

```bash
moon run main
```

应可看到输出 `Hello, world!`

### 交互模式

和 Rust 版本不同，MoonBit 没有提供对 tty 的判断。因此只需要在设置 `NO_COLOR`
环境变量时进入测试模式，否则进入交互模式即可。

### 随机数

请使用 `./io` 文件夹中提供的 `rand` 包，以确保行为与Rust测试程序一致

### JSON 序列化

MoonBit 标准库提供了 `@json`
可用于配置和游戏状态的序列化、反序列化，具体用法可参考标准库文档。

## 自动测试

自动测试依然由 Rust 代码驱动。为了区分两种语言，如果使用 MoonBit
编写，请在项目根目录下创建 `.test_moon`
文件，并提交到仓库。除此之外，也可以通过设置 `TEST_MOON` 环境变量来指定使用
MoonBit 进行测试。

在命令行中，直接执行 `TEST_MOON=1 cargo test` 即可以 MoonBit 语言运行所有测试。

如果某个测试用例失败，可以通过 Rust Analyzer 插件在
`../tests/basic_requirements.rs` 目录下查看测试用例的内容，点击 `Run Test`
运行单个测试用例（由于图形界面不支持传入环境变量，此时必须创建 `.test_moon`
文件）。

也可以使用如下的命令，在 `wordle-mbt` 目录下手工测试某些点：

```bash
moon run main -- -w build < ../tests/cases/02_01_specify_answer.in > ../tests/cases/02_01_specify_answer.out
diff --color ../tests/cases/02_01_specify_answer.ans ../tests/cases/02_01_specify_answer.out
```

如遇到程序崩溃，可以添加 `--debug` 参数，用于打印调用栈的函数名

```bash
moon build --debug
moon run main --debug
```

## 其他

如遇到与 MoonBit 语言、工具链相关的问题，可在 <https://taolun.moonbitlang.cn/>
论坛中提问。
