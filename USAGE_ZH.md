# 如何使用这个模板

这个模板项目包含了 Rust 项目的四大基础组件：
1. **命令行参数解析** (使用 clap)
2. **配置管理** (使用 config)
3. **结构化日志** (使用 tracing)
4. **测试用例** (单元测试和集成测试)

## 方法一: 作为 cargo-generate 模板使用

### 1. 安装 cargo-generate
```bash
cargo install cargo-generate
```

### 2. 将此项目推送到 Git 仓库
```bash
git init
git add .
git commit -m "Initial commit: Rust project template"
git remote add origin <你的仓库地址>
git push -u origin main
```

### 3. 使用模板创建新项目
```bash
cargo generate --git <你的仓库地址>
# 或者如果是本地路径
cargo generate --path D:\works\rusts\project-tpl
```

## 方法二: 直接复制使用

### 1. 复制整个项目目录
```bash
# Windows PowerShell
Copy-Item -Recurse D:\works\rusts\project-tpl D:\works\rusts\my-new-project

# Linux/Mac
cp -r /path/to/project-tpl /path/to/my-new-project
```

### 2. 修改项目名称
编辑 `Cargo.toml`:
```toml
[package]
name = "my-new-project"  # 改成你的项目名
version = "0.1.0"
edition = "2021"

[lib]
name = "my_new_project"  # 改成你的库名 (使用下划线)
path = "src/lib.rs"

[[bin]]
name = "my-new-project"  # 改成你的二进制名
path = "src/main.rs"
```

### 3. 更新配置文件
编辑 `config/default.toml`:
```toml
[app]
name = "my-new-project"
version = "0.1.0"
```

### 4. 清理和构建
```bash
cd my-new-project
cargo clean
cargo build
cargo test
```

## 方法三: 使用 git clone 和手动设置

### 1. Clone 这个仓库
```bash
git clone <仓库地址> my-new-project
cd my-new-project
```

### 2. 移除原有的 git 历史
```bash
# Windows
Remove-Item -Recurse -Force .git

# Linux/Mac
rm -rf .git
```

### 3. 初始化新的 git 仓库
```bash
git init
git add .
git commit -m "Initial commit based on template"
```

### 4. 按照方法二的步骤修改项目名称

## 快速开始

创建项目后，你可以：

### 运行项目
```bash
# 开发模式运行
cargo run

# 详细输出模式
cargo run -- --verbose

# 指定环境
cargo run -- --env production

# 查看帮助
cargo run -- --help
```

### 运行测试
```bash
# 运行所有测试
cargo test

# 运行测试并显示输出
cargo test -- --nocapture

# 运行特定测试
cargo test test_config_loading
```

### 构建发布版本
```bash
cargo build --release
```

## 项目结构说明

```
project-tpl/
├── config/              # 配置文件目录
│   ├── default.toml    # 默认配置
│   ├── development.toml # 开发环境配置
│   └── production.toml  # 生产环境配置
├── logs/               # 日志输出目录
├── src/
│   ├── main.rs        # 应用程序入口
│   ├── lib.rs         # 库导出
│   ├── cli.rs         # 命令行参数定义
│   ├── config.rs      # 配置管理
│   └── logging.rs     # 日志系统初始化
├── tests/
│   └── integration_test.rs # 集成测试
├── .env.example       # 环境变量示例
├── .gitignore        # Git 忽略文件
├── Cargo.toml        # 依赖配置
├── Makefile          # 常用命令（可选）
└── README.md         # 项目说明
```

## 自定义模板

### 添加新的配置项
1. 编辑 `src/config.rs`，添加新的配置结构
2. 更新 `config/default.toml`，添加配置值
3. 在代码中使用 `config.your_new_setting`

### 添加新的 CLI 参数
1. 编辑 `src/cli.rs`，在 `Cli` 结构中添加新字段
2. 使用 `#[arg(...)]` 属性定义参数
3. 在 `main.rs` 中访问 `cli.your_new_arg`

### 修改日志格式
编辑 `config/default.toml`:
```toml
[logging]
level = "debug"      # 可选: trace, debug, info, warn, error
format = "json"      # 可选: json, text
file_output = true   # 是否输出到文件
console_output = true # 是否输出到控制台
```

## 常见问题

**Q: 如何添加新的依赖？**
```bash
cargo add <dependency-name>
```

**Q: 如何更改日志级别？**
```bash
# 方法1: 命令行参数
cargo run -- --log-level debug

# 方法2: 环境变量
$env:LOG_LEVEL="debug"; cargo run

# 方法3: 修改配置文件
# 编辑 config/development.toml
```

**Q: 如何添加子命令？**
编辑 `src/cli.rs`，在 `Commands` 枚举中添加新变体。

## 进阶使用

### 使用环境变量覆盖配置
```bash
# 设置服务器端口
$env:APP_SERVER__PORT="9090"
cargo run

# 设置数据库连接
$env:APP_DATABASE__URL="postgres://localhost/newdb"
cargo run
```

### 使用 Makefile（需要 make 工具）
```bash
make build    # 构建项目
make test     # 运行测试
make run      # 运行项目
make dev      # 开发模式运行
make clean    # 清理构建产物
```

## 下一步

1. 根据你的需求修改配置结构
2. 实现具体的业务逻辑
3. 添加更多测试用例
4. 完善错误处理
5. 添加文档注释

祝你使用愉快！🎉
