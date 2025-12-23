---
name: agent-skills-expert
description: Expert for creating and managing Claude Code Agent Skills. Create skills with git submodule + sparse-checkout for source references, write SKILL.md with proper frontmatter, and follow best practices. Use when creating new skills, adding source references to skills, or managing skill configurations.
---

# Agent Skills Expert

专业的 Claude Code Agent Skills 创建和管理专家。帮助创建符合官方规范的技能，包括使用 Git Submodule + Sparse Checkout 管理源码引用。

## 📚 核心概念

### 什么是 Agent Skill？

Agent Skill 是 Claude Code 的可扩展能力模块，包含：
- `SKILL.md` - 技能定义文件（必需）
- 支持文件 - 文档、脚本、模板等（可选）
- `source/` - 源码引用目录（可选，使用 git submodule）

### 技能存储位置

- **个人技能**: `~/.claude/skills/skill-name/`
- **项目技能**: `.claude/skills/skill-name/`

## 🔧 创建技能流程

### 1. 基础技能结构

```
skill-name/
├── SKILL.md              # 必需：技能定义
├── examples.md           # 可选：示例代码
├── quick-reference.md    # 可选：快速参考
├── SOURCE_STRUCTURE.md   # 可选：源码结构文档
├── scripts/              # 可选：辅助脚本
└── source/               # 可选：源码引用
    └── repo-name/        # Git Submodule
```

### 2. SKILL.md 规范

```yaml
---
name: skill-name                    # 必需：小写字母、数字、连字符，最多64字符
description: Brief description...   # 必需：技能描述，最多1024字符
allowed-tools: Read, Grep, Glob     # 可选：限制可用工具
---

# Skill Name

## Instructions
清晰的使用说明...

## Examples
具体的使用示例...
```

**description 最佳实践**：
- 说明技能做什么
- 说明何时使用该技能
- 包含关键词便于 Claude 发现

### 3. 添加源码引用（Git Submodule + Sparse Checkout）

#### 步骤 1：添加 Submodule

```bash
cd ~/.claude/skills
git submodule add https://github.com/org/repo.git skill-name/source/repo-name
```

#### 步骤 2：配置 Sparse Checkout

```bash
cd skill-name/source/repo-name

# 初始化 sparse-checkout
git sparse-checkout init --no-cone

# 设置要保留的内容
git sparse-checkout set \
    /README.md \
    /docs/ \
    /src/ \
    /examples/
```

#### 步骤 3：创建 SOURCE_STRUCTURE.md

文档化源码结构，包括：
- Sparse checkout 配置
- 目录结构说明
- 关键文件位置
- 维护指南

## 📋 Sparse Checkout 配置指南

### 选择保留内容的原则

1. **核心源码** - 主要 API 实现
2. **文档** - README、docs 目录
3. **示例** - examples 目录
4. **配置文件** - pyproject.toml、package.json 等
5. **测试** - 展示使用模式的测试用例

### 排除的内容

- 大型资源文件（图片、视频）
- 构建产物
- CI/CD 配置（通常不需要）
- 历史发布说明

### 常用 Sparse Checkout 模式

**Python 项目：**
```bash
git sparse-checkout set \
    /README.md \
    /LICENSE \
    /src/ \
    /docs/ \
    /examples/ \
    /tests/ \
    /pyproject.toml
```

**JavaScript/TypeScript 项目：**
```bash
git sparse-checkout set \
    /README.md \
    /LICENSE \
    /src/ \
    /docs/ \
    /examples/ \
    /package.json \
    /tsconfig.json
```

**Rust 项目：**
```bash
git sparse-checkout set \
    /README.md \
    /LICENSE \
    /src/ \
    /docs/ \
    /examples/ \
    /Cargo.toml
```

## 🛠️ 维护操作

### 更新 Submodule

```bash
cd skill-name/source/repo-name
git pull origin main
```

### 修改 Sparse Checkout 配置

```bash
cd skill-name/source/repo-name

# 添加新目录
git sparse-checkout add /new-dir/

# 重新设置
git sparse-checkout set /dir1/ /dir2/ /file.md
```

### 查看配置

```bash
cd skill-name/source/repo-name
git sparse-checkout list
du -sh .  # 查看大小
```

### 故障恢复

```bash
# 完全重置 submodule
cd ~/.claude/skills
git submodule deinit -f skill-name/source/repo-name
rm -rf .git/modules/skill-name/source/repo-name
git submodule update --init skill-name/source/repo-name

# 重新配置 sparse-checkout
cd skill-name/source/repo-name
git sparse-checkout init --no-cone
git sparse-checkout set /保留的目录/
```

## 📝 模板文件

### SKILL.md 模板

见 [templates/SKILL_TEMPLATE.md](templates/SKILL_TEMPLATE.md)

### SOURCE_STRUCTURE.md 模板

见 [templates/SOURCE_STRUCTURE_TEMPLATE.md](templates/SOURCE_STRUCTURE_TEMPLATE.md)

## ✅ 检查清单

### 创建新技能

- [ ] 创建技能目录 `mkdir -p ~/.claude/skills/skill-name`
- [ ] 创建 SKILL.md（包含正确的 frontmatter）
- [ ] 编写清晰的 description（包含触发词）
- [ ] 添加使用说明和示例
- [ ] 测试技能是否被正确发现

### 添加源码引用

- [ ] 添加 git submodule
- [ ] 配置 sparse-checkout
- [ ] 验证大小合理（通常 <100MB）
- [ ] 创建 SOURCE_STRUCTURE.md
- [ ] 更新 SKILL.md 中的源码访问说明
- [ ] 提交所有更改

### 维护

- [ ] 定期更新 submodule
- [ ] 检查 sparse-checkout 配置是否仍然合适
- [ ] 更新文档反映最新结构

## 🔗 相关资源

- [GIT_SPARSE_CHECKOUT_TUTORIAL.md](../GIT_SPARSE_CHECKOUT_TUTORIAL.md) - Sparse Checkout 详细教程
- [CLAUDE_CODE_SKILL_TUTORIAL.md](../CLAUDE_CODE_SKILL_TUTORIAL.md) - 官方技能教程
- [Agent Skills 官方文档](https://docs.anthropic.com/en/docs/agents-and-tools/agent-skills/overview)

## 📊 现有技能参考

| 技能 | 源码引用 | Sparse Checkout |
|------|----------|-----------------|
| langgraph-python-expert | ✅ | ✅ (~66MB) |
| lib-slint-expert | ✅ | ❌ |
| vscode-extension-builder | ✅ | ✅ |
| uv-expert | ✅ | ❌ |
| rust-cli-tui-developer | ✅ | ❌ |

---

*最后更新: 2024-12-23*

