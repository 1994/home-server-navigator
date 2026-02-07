# Contributing to Home Server Navigator

感谢你对 Home Server Navigator 项目的关注！我们欢迎所有形式的贡献，包括 bug 报告、功能建议、代码贡献和文档改进。

## 如何贡献

### 报告 Bug

如果你发现了 bug，请通过 [GitHub Issues](https://github.com/YOUR_USERNAME/home-server-navigator/issues) 提交报告。报告时请包含以下信息：

- 问题的清晰描述
- 复现步骤
- 期望行为 vs 实际行为
- 系统环境（操作系统、Node 版本、Rust 版本等）
- 相关日志或错误信息

### 建议新功能

如果你有新功能的想法，请先：

1. 检查是否已有相关的 issue
2. 如果没有，创建一个新的 issue，描述你的功能建议
3. 等待维护者的反馈后再开始编码

### 提交代码

1. **Fork 仓库** - 点击仓库右上角的 Fork 按钮

2. **克隆你的 Fork**
   ```bash
   git clone https://github.com/YOUR_USERNAME/home-server-navigator.git
   cd home-server-navigator
   ```

3. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/your-bug-fix
   ```

4. **进行更改**
   - 确保代码符合项目风格
   - 添加必要的测试
   - 更新相关文档

5. **测试你的更改**
   ```bash
   # 构建测试
   make build
   
   # 运行后端测试
   cd backend && cargo test
   
   # 前端类型检查
   cd frontend && npm run build
   ```

6. **提交更改**
   ```bash
   git add .
   git commit -m "feat: add some feature"
   # 或
   git commit -m "fix: fix some bug"
   ```

   提交信息格式建议：
   - `feat:` 新功能
   - `fix:` Bug 修复
   - `docs:` 文档更新
   - `style:` 代码格式调整（不影响功能）
   - `refactor:` 代码重构
   - `perf:` 性能优化
   - `test:` 测试相关
   - `chore:` 构建过程或辅助工具的变动

7. **推送到你的 Fork**
   ```bash
   git push origin feature/your-feature-name
   ```

8. **创建 Pull Request**
   - 前往原仓库页面
   - 点击 "New Pull Request"
   - 选择你的分支
   - 填写 PR 描述，说明你的更改

## 开发指南

### 项目结构

```
backend/
├── src/
│   ├── main.rs        # 程序入口
│   ├── api.rs         # HTTP 路由处理
│   ├── discovery.rs   # 服务发现逻辑
│   ├── models.rs      # 数据模型定义
│   ├── state.rs       # 应用状态管理
│   └── store.rs       # 数据持久化
└── Cargo.toml

frontend/
├── src/
│   ├── App.tsx        # 主组件
│   ├── components/    # 可复用组件
│   ├── pages/         # 页面组件
│   └── api/           # API 客户端
└── package.json
```

### 后端开发 (Rust)

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码
- 遵循 Rust 官方编码规范
- 添加适当的错误处理

### 前端开发 (React + TypeScript)

- 使用 TypeScript 严格模式
- 组件使用函数式组件 + Hooks
- 遵循 React 最佳实践
- 保持代码简洁可读

### 代码风格检查

```bash
# 后端
make -C backend check
make -C backend fmt
make -C backend clippy

# 前端
cd frontend && npm run lint
```

## 发布流程

1. 更新版本号（`backend/Cargo.toml` 和 `frontend/package.json`）
2. 更新 `CHANGELOG.md`
3. 创建新的 git tag
   ```bash
   git tag -a v1.0.0 -m "Release version 1.0.0"
   git push origin v1.0.0
   ```
4. GitHub Actions 会自动构建并发布 Release

## 社区规范

- 保持友善和尊重
- 接受建设性的批评
- 关注对社区最有利的事情
- 尊重不同的观点和经验

## 获取帮助

如果你需要帮助，可以通过以下方式联系：

- 在 GitHub Discussions 中发起讨论
- 在相关 issue 中评论
- 发送邮件到 [your-email@example.com]

再次感谢你的贡献！
