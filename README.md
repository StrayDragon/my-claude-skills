# StrayDragon Claude Skills

[![Plugin Marketplace](https://img.shields.io/badge/Plugin-Marketplace-blue)](https://github.com/StrayDragon/straydragon-claude-skills)
[![Claude Code](https://img.shields.io/badge/Claude-Code-purple)](https://claude.ai/claude-code)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

> 🚀 **StrayDragon 的 Claude 技能市场** - 精心整理的实用 Claude 技能集合，涵盖 Python 开发、GUI 编程、测试、日志、包管理和 VSCode 扩展开发等领域

A curated collection of practical Claude Skills for enhancing productivity across Python development, GUI programming, testing, logging, package management, and VSCode extension development.

## ✨ 特色技能 | Featured Skills

### 🐍 Python 开发技能 | Python Development Skills

| 技能 | 描述 | 适用场景 |
|------|------|----------|
| **[langgraph-python-expert](./langgraph-python-expert/)** | LangGraph Python 专家指导，构建状态化多智能体 LLM 应用 | 构建复杂工作流、多智能体系统 |
| **[python-expert-tester](./python-expert-tester/)** | Python 测试专家，精通 pytest 和异步测试 | 编写高质量测试、提升测试覆盖率 |
| **[python-log-expert](./python-log-expert/)** | Python 日志专家，包含 structlog 源码分析 | 日志系统设计、故障排查 |
| **[uv-expert](./uv-expert/)** | uv Python 包和项目管理专家 | 项目迁移、依赖管理优化 |

### 🖥️ GUI 与扩展开发 | GUI & Extension Development

| 技能 | 描述 | 适用场景 |
|------|------|----------|
| **[lib-slint-expert](./lib-slint-expert/)** | Slint GUI 开发专家，基于官方源码 | 跨平台原生 GUI 应用开发 |
| **[vscode-extension-builder](./vscode-extension-builder/)** | VSCode 扩展开发专家，包含完整 API 参考 | 编辑器扩展开发、工具链定制 |

## 🚀 快速安装 | Quick Installation

### 方法一：从 GitHub 仓库安装（推荐）

```bash
# 添加此仓库为插件市场
/plugin marketplace add https://github.com/StrayDragon/straydragon-claude-skills

# 浏览可用插件
/plugin

# 安装单个技能
/plugin install langgraph-python-expert@straydragon-claude-skills
/plugin install python-expert-tester@straydragon-claude-skills
/plugin install python-log-expert@straydragon-claude-skills
/plugin install uv-expert@straydragon-claude-skills
/plugin install lib-slint-expert@straydragon-claude-skills
/plugin install vscode-extension-builder@straydragon-claude-skills
```

### 方法二：本地开发测试

```bash
# 克隆仓库
git clone https://github.com/StrayDragon/straydragon-claude-skills.git
cd straydragon-claude-skills

# 添加为本地市场
/plugin marketplace add .

# 安装技能进行测试
/plugin install langgraph-python-expert@straydragon-claude-skills
```

## 💡 使用指南 | Usage Guide

### 自动技能触发 | Automatic Skill Activation

每个技能都会根据您的请求自动激活。无需手动调用，Claude 会智能选择合适的技能：

**中文示例：**
- **LangGraph**: "帮我用 LangGraph 构建一个多智能体工作流"
- **测试**: "为这个 Python 模块编写全面的测试"
- **日志**: "帮我设置基于 structlog 的结构化日志"
- **uv**: "帮我把这个项目迁移到使用 uv"
- **Slint**: "用 Rust 和 Slint 创建一个 GUI 应用"
- **VSCode 扩展**: "帮我开发一个 VSCode 扩展"

**English Examples:**
- **LangGraph**: "Help me build a multi-agent workflow with LangGraph"
- **Testing**: "Write comprehensive tests for this Python module"
- **Logging**: "Set up structured logging with structlog"
- **uv**: "Help me migrate this project to use uv"
- **Slint**: "Create a Slint GUI application with Rust"
- **VSCode Extensions**: "Help me create a VSCode extension"

### 技能特性 | Skill Features

- ✅ **模型调用** - Claude 根据上下文自动选择合适的技能
- ✅ **完整源码** - 包含相关库的完整源代码用于深度分析
- ✅ **最佳实践** - 遵循行业标准和最佳实践
- ✅ **实时更新** - 与上游库保持同步更新

## 🛠️ 安装要求 | Requirements

### 系统要求
- Claude Code 1.0 或更高版本
- Git（用于克隆仓库）

### Python 技能依赖
- **Python 3.10+**（推荐 3.11+）
- 相关包会在使用时自动安装提示

### 可选依赖
- **Rust**（用于 Slint 开发）
- **Node.js**（用于 VSCode 扩展开发）

## 📚 技能详解 | Skill Details

### Python 技能系列

<details>
<summary><strong>🔧 langgraph-python-expert</strong></summary>

**功能**: LangGraph Python 库专家指导
**核心特性**:
- 状态化多智能体应用设计
- 节点和边的最佳实践
- 复杂工作流构建
- 错误处理和调试技巧

**使用场景**: 构建需要状态管理的复杂 LLM 应用
**依赖**: `langgraph`, `langchain-core`
</details>

<details>
<summary><strong>🧪 python-expert-tester</strong></summary>

**功能**: Python 测试专家指导
**核心特性**:
- pytest 高级用法
- 异步测试策略
- 测试覆盖率优化
- 测试架构设计

**使用场景**: 编写高质量的 Python 测试套件
**依赖**: `pytest`, `pytest-asyncio`
</details>

<details>
<summary><strong>📝 python-log-expert</strong></summary>

**功能**: Python 日志专家，包含 structlog 完整源码
**核心特性**:
- 结构化日志设计
- 性能优化技巧
- 日志分析和调试
- 完整 structlog 源码参考

**使用场景**: 设计可扩展的日志系统，排查复杂日志问题
**依赖**: `structlog`, `logging`
**亮点**: 包含 structlog 完整源代码，支持深度实现分析
</details>

<details>
<summary><strong>⚡ uv-expert</strong></summary>

**功能**: uv Python 包管理专家
**核心特性**:
- 项目配置和依赖管理
- 性能优化技巧
- 从 pip/venv 迁移指南
- 跨平台兼容性

**使用场景**: 现代化 Python 项目工具链
**依赖**: `uv`
**优势**: 相比传统 pip + venv 提供显著性能提升
</details>

### GUI 与扩展开发技能

<details>
<summary><strong>🎨 lib-slint-expert</strong></summary>

**功能**: Slint GUI 开发专家
**核心特性**:
- Rust 集成开发
- 响应式布局设计
- 动画和特效
- 跨平台部署

**使用场景**: 开发高性能原生 GUI 应用
**依赖**: Rust, Slint 工具链
**特色**: 基于 Slint 官方源码，提供权威指导
</details>

<details>
<summary><strong>🔌 vscode-extension-builder</strong></summary>

**功能**: VSCode 扩展开发专家
**核心特性**:
- Yeoman 生成器使用
- TypeScript API 参考
- 调试和发布流程
- 完整官方文档

**使用场景**: 定制编辑器功能，提升开发效率
**依赖**: Node.js, yo, VS Code Extension CLI
**资源**: 包含完整 VS Code API 文档
</details>

## 🤝 贡献指南 | Contributing

我们欢迎各种形式的贡献！

### 快速贡献流程

1. **Fork 仓库**
   ```bash
   # 在 GitHub 上 Fork，然后克隆
   git clone https://github.com/your-username/straydragon-claude-skills.git
   cd straydragon-claude-skills
   git remote add upstream https://github.com/StrayDragon/straydragon-claude-skills.git
   ```

2. **创建技能目录**
   ```bash
   mkdir your-new-skill
   cd your-new-skill
   ```

3. **编写技能文件**
   - 创建 `SKILL.md` 并包含适当的前置元数据
   - 创建 `.claude-plugin/plugin.json` 插件配置
   - 添加源码、脚本或示例（可选）

4. **测试技能**
   ```bash
   # 本地测试
   /plugin marketplace add .
   /plugin install your-new-skill@straydragon-claude-skills
   ```

5. **提交 PR**
   ```bash
   git add .
   git commit -m "feat: add your-new-skill"
   git push origin main
   # 然后在 GitHub 上创建 Pull Request
   ```

### 技能开发规范

#### 目录结构
```
your-skill/
├── SKILL.md                 # 主技能文档（必需）
├── .claude-plugin/
│   └── plugin.json         # 插件元数据（必需）
├── source/                 # 源代码（可选）
│   └── library-name/       # 相关库的完整源码
├── scripts/               # 实用脚本（可选）
├── examples/              # 使用示例（可选）
└── docs/                  # 额外文档（可选）
```

#### SKILL.md 规范
```yaml
---
name: your-skill-name
description: 技能描述，包含使用场景和触发关键词
---
```

#### plugin.json 规范
```json
{
  "name": "your-skill-name",
  "description": "简短描述",
  "version": "1.0.0",
  "author": {
    "name": "Your Name",
    "url": "https://github.com/your-username"
  },
  "categories": ["development", "python"],
  "tags": ["tag1", "tag2"]
}
```

### 贡献类型

- 🐛 **Bug 修复**: 修复现有技能的问题
- ✨ **新技能**: 添加全新的技能领域
- 📚 **文档改进**: 完善 README 和技能文档
- 🔧 **工具脚本**: 添加辅助开发脚本
- 🧪 **测试覆盖**: 为技能添加测试用例

## 📄 许可证 | License

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 👨‍💻 作者 | Author

由 [StrayDragon](https://github.com/StrayDragon) 创建和维护。

- **GitHub**: [@StrayDragon](https://github.com/StrayDragon)
- **邮箱**: l8ng@proton.me

## 🙏 致谢 | Acknowledgments

- 感谢 [Anthropic](https://anthropic.com) 提供的 Claude Code 平台
- 各技能对应的开源项目维护者
- 社区贡献者的反馈和改进建议

## 📖 相关资源 | Resources

- [Claude Skills 官方文档](https://docs.claude.com/en/docs/agents-and-tools/agent-skills)
- [Claude Code 插件开发指南](https://docs.claude.com/en/docs/claude-code/plugins)
- [插件市场管理](https://docs.claude.com/en/plugin-marketplaces)

---

<div align="center">

**🌟 如果这个技能市场对您有帮助，请考虑给仓库点个 Star！**

Made with ❤️ by StrayDragon

</div>