# StrayDragon Claude Skills

## 安装

### 方法一：从 GitHub 仓库安装

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

### 方法二：本地安装

```bash
# 克隆仓库
git clone https://github.com/StrayDragon/straydragon-claude-skills.git
cd straydragon-claude-skills

# 添加为本地市场
/plugin marketplace add .

# 安装技能
/plugin install langgraph-python-expert@straydragon-claude-skills
```

## 使用

技能会根据您的请求自动激活。示例：
- "帮我用 LangGraph 构建一个多智能体工作流"
- "为这个 Python 模块编写全面的测试"
- "帮我把这个项目迁移到使用 uv"
- "用 Rust 和 Slint 创建一个 GUI 应用"

## 技能列表

- **langgraph-python-expert**: LangGraph Python 专家指导
- **python-expert-tester**: Python 测试专家，精通 pytest
- **python-log-expert**: Python 日志专家，包含 structlog 源码
- **uv-expert**: uv Python 包和项目管理专家
- **lib-slint-expert**: Slint GUI 开发专家
- **vscode-extension-builder**: VSCode 扩展开发专家