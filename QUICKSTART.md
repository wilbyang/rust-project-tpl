# Quick Start Guide

## 🚀 快速开始

### 使用此模板的三种方法

#### 方法 1️⃣: 使用 cargo-generate（推荐）

```bash
# 1. 安装 cargo-generate
cargo install cargo-generate

# 2. 从本地路径创建新项目
cargo generate --path D:\works\rusts\project-tpl --name my-project

# 或从 Git 仓库创建（需先上传到 Git）
cargo generate --git https://github.com/your-username/project-tpl --name my-project
```

#### 方法 2️⃣: 直接复制

```bash
# Windows PowerShell
Copy-Item -Recurse D:\works\rusts\project-tpl D:\works\rusts\my-project
cd D:\works\rusts\my-project

# 修改 Cargo.toml 中的项目名称
# 然后运行
cargo clean
cargo build
```

#### 方法 3️⃣: Git Clone（如果已上传）

```bash
git clone <repo-url> my-project
cd my-project
rm -rf .git
git init
```

---

## ✅ 验证安装

```bash
# 构建项目
cargo build

# 运行测试（18个测试应该全部通过）
cargo test

# 运行程序
cargo run

# 查看帮助
cargo run -- --help
```

---

## 📦 项目包含的功能

### 1. 命令行参数 (CLI)
- ✅ 使用 `clap` 解析参数
- ✅ 支持子命令 (serve, config, migrate)
- ✅ 环境变量集成
- ✅ 自动生成帮助信息

```bash
cargo run -- --verbose
cargo run -- --env production
cargo run -- serve --port 8080
```

### 2. 配置管理
- ✅ 多环境配置 (development, production)
- ✅ TOML 格式配置文件
- ✅ 环境变量覆盖
- ✅ 配置验证

```bash
# 使用环境变量覆盖配置
$env:APP_SERVER__PORT="9090"
cargo run
```

### 3. 结构化日志
- ✅ 使用 `tracing` 框架
- ✅ 支持 JSON 和文本格式
- ✅ 控制台和文件输出
- ✅ 日志文件每日轮换
- ✅ 可配置日志级别

```bash
# 设置日志级别
cargo run -- --log-level debug
$env:LOG_LEVEL="trace"
cargo run
```

### 4. 测试用例
- ✅ 单元测试（在各模块内）
- ✅ 集成测试（tests/ 目录）
- ✅ 配置测试
- ✅ CLI 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_config_loading

# 显示测试输出
cargo test -- --nocapture
```

---

## 📁 项目结构

```
project-tpl/
├── src/
│   ├── main.rs         # 程序入口，初始化四大件
│   ├── lib.rs          # 库导出
│   ├── cli.rs          # CLI 参数定义
│   ├── config.rs       # 配置管理
│   └── logging.rs      # 日志系统
├── config/
│   ├── default.toml    # 默认配置
│   ├── development.toml # 开发环境
│   └── production.toml  # 生产环境
├── tests/
│   └── integration_test.rs # 集成测试
├── logs/               # 日志输出目录
├── .env.example        # 环境变量示例
├── Cargo.toml          # 依赖配置
└── README.md           # 完整文档
```

---

## 🔧 常用命令

```bash
# 开发
cargo run                    # 运行程序
cargo run -- --verbose       # 详细输出
cargo test                   # 运行测试
cargo check                  # 检查代码

# 构建
cargo build                  # Debug 构建
cargo build --release        # Release 构建

# 代码质量
cargo fmt                    # 格式化代码
cargo clippy                 # 代码检查

# 清理
cargo clean                  # 清理构建产物
```

---

## 📝 自定义你的项目

### 修改项目名称
编辑 `Cargo.toml`:
```toml
name = "your-project-name"
```

### 添加新配置
1. 编辑 `src/config.rs` - 添加配置结构
2. 编辑 `config/default.toml` - 添加配置值
3. 在代码中使用配置

### 添加新的 CLI 参数
编辑 `src/cli.rs`:
```rust
pub struct Cli {
    #[arg(long)]
    pub your_arg: String,
}
```

### 实现业务逻辑
在 `src/main.rs` 的 `run_application` 函数中添加你的代码。

---

## 🎯 下一步

1. **修改配置**: 根据你的需求调整 `config/default.toml`
2. **实现业务**: 在 `run_application` 中编写你的逻辑
3. **添加测试**: 为新功能编写测试用例
4. **完善文档**: 更新 README.md

---

## 📚 依赖说明

| 依赖 | 用途 | 版本 |
|-----|------|------|
| clap | 命令行参数解析 | 4.5 |
| config | 配置管理 | 0.14 |
| tracing | 结构化日志 | 0.1 |
| serde | 序列化/反序列化 | 1.0 |
| anyhow | 错误处理 | 1.0 |

---

## ❓ 常见问题

**Q: 如何改变日志输出位置？**
A: 编辑 `src/logging.rs` 中的 `.build("logs")` 行

**Q: 如何添加新的环境配置？**
A: 在 `config/` 目录创建新的 `.toml` 文件，如 `test.toml`

**Q: 测试失败怎么办？**
A: 运行 `cargo test -- --nocapture` 查看详细输出

---

## 🎉 完成！

现在你有了一个完整的 Rust 项目模板，包含：
- ✅ 命令行参数解析
- ✅ 配置文件管理
- ✅ 结构化日志系统
- ✅ 完整的测试框架

开始编写你的应用程序吧！💪
