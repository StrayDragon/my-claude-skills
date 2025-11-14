# StrayDragon Claude Skills

## 安装前检查

**请先检查网络连接**：确保可以访问 GitHub，因为安装过程需要下载子模块。

```bash
curl -I https://github.com
```

## 安装方法

### 重要说明

此仓库包含 Git 子模块，需要额外步骤才能完整安装：

- **方法一**：快速安装，部分技能功能受限
- **方法二**：完整安装，推荐使用（包含所有源码文件）

---

### 方法一：快速安装（功能受限）

适用于快速体验，但以下技能将缺少源码文件：
- `python-log-expert`（缺少 structlog 源码）
- `lib-slint-expert`（缺少 Slint 官方源码）
- `uv-expert`（缺少 uv 源码）
- `vscode-extension-builder`（缺少 VSCode 官方文档）

```bash
# 添加此仓库为插件市场
/plugin marketplace add https://github.com/StrayDragon/straydragon-claude-skills

# 安装技能
/plugin install langgraph-python-expert@straydragon-claude-skills
/plugin install python-expert-tester@straydragon-claude-skills
/plugin install python-log-expert@straydragon-claude-skills
/plugin install uv-expert@straydragon-claude-skills
/plugin install lib-slint-expert@straydragon-claude-skills
/plugin install vscode-extension-builder@straydragon-claude-skills
```

**注意**：使用此方法后，如果发现某些技能功能不完整，请改用方法二重新安装。

---

### 方法二：完整安装（推荐）

提供完整功能，包含所有必要的源码文件和依赖。

```bash
# 1. 克隆仓库
git clone https://github.com/StrayDragon/straydragon-claude-skills.git
cd straydragon-claude-skills

# 2. 初始化并更新所有子模块（关键步骤）
git submodule update --init --recursive

# 3. 验证子模块下载成功
# 手动检查：ls -la */source/

# 4. 添加为本地市场
/plugin marketplace add .

# 5. 安装所需技能
/plugin install langgraph-python-expert@straydragon-claude-skills
/plugin install python-expert-tester@straydragon-claude-skills
/plugin install python-log-expert@straydragon-claude-skills
/plugin install uv-expert@straydragon-claude-skills
/plugin install lib-slint-expert@straydragon-claude-skills
/plugin install vscode-extension-builder@straydragon-claude-skills
```

**网络问题排查**：如果 `git submodule update` 失败，检查网络连接或尝试使用代理。

---

## 使用方法

技能会根据您的请求自动激活。示例：

```bash
# LangGraph 相关
"帮我用 LangGraph 构建一个多智能体工作流"

# Python 测试
"为这个 Python 模块编写全面的测试"

# 日志处理
"如何使用 structlog 进行结构化日志记录"

# 项目管理
"帮我把这个项目迁移到使用 uv"

# GUI 开发
"用 Rust 和 Slint 创建一个 GUI 应用"

# VSCode 扩展
"我想开发一个 VSCode 扩展"
```

## 技能详情

| 技能名称 | 描述 | 源码类型 | 安装要求 |
|---------|------|---------|---------|
| **langgraph-python-expert** | LangGraph Python 专家指导 | 自包含 | 两种方法均可 |
| **python-expert-tester** | Python 测试专家，精通 pytest | 自包含 | 两种方法均可 |
| **python-log-expert** | Python 日志专家，包含 structlog 源码 | Git Submodule | 需完整安装 |
| **uv-expert** | uv Python 包和项目管理专家 | Git Submodule | 需完整安装 |
| **lib-slint-expert** | Slint GUI 开发专家 | Git Submodule | 需完整安装 |
| **vscode-extension-builder** | VSCode 扩展开发专家 | Git Submodule | 需完整安装 |

---

## 故障排除

### 常见问题

**Q: 技能安装成功但功能不完整？**
A: 可能是子模块未正确下载。请使用方法二重新安装，确保 `git submodule update --init --recursive` 成功执行。

**Q: git submodule 更新失败？**
A: 检查网络连接，或尝试：
```bash
git submodule deinit --all
git submodule update --init --recursive
```

**Q: 某个技能无法激活？**
A: 检查技能目录结构是否完整，特别是 source/ 子目录是否包含文件。

### 验证安装

```bash
# 1. 检查技能是否正确安装
/agents

# 2. 检查技能可用性
"What Skills are available?"

# 3. 测试特定技能
"帮我分析这个 structlog 日志配置"
```

