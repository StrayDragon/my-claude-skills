# Git Sparse Checkout 教程与维护指南

## 🎯 什么是 Sparse Checkout？

Sparse Checkout（稀疏检出）是 Git 的一个功能，允许你只检出仓库中的特定文件或目录，而不是整个仓库的内容。这对于大型仓库或只需要部分内容的场景非常有用。

### 📚 核心概念

- **完整仓库**: Git 仍然下载完整的仓库历史和元数据（.git 目录）
- **工作区过滤**: 只在工作区显示你指定的文件/目录
- **独立配置**: 每个 submodule 的 sparse checkout 配置是独立的

## 🔧 本次优化的配置详情

### VSCode Docs 配置

**位置**: `.git/modules/vscode-extension-builder/source/vscode-docs/info/sparse-checkout`

**当前配置**:
```
api/
docs/
learn/
CONTRIBUTING.md
LICENSE.md
README.md
SECURITY.md
```

**保留内容**:
- `api/` - VS Code API 参考文档
- `docs/` - 核心开发文档（只保留基础部分）
- `learn/` - 学习资源
- `*.md` - 重要的 Markdown 文档

**移除的大型内容**:
- `release-notes/` (1.2GB) - 历史发布说明
- `blogs/` (390MB) - 博客文章
- `docs/` 中大部分语言特定文档

### VSCode Extension Samples 配置

**位置**: `.git/modules/vscode-extension-builder/source/vscode-extension-samples/info/sparse-checkout`

**当前配置**:
```
helloworld-sample
helloworld-minimal-sample
getting-started-sample
completions-sample
code-actions-sample
lsp-sample
package.json
README.md
LICENSE
```

**保留内容**:
- 基础 TypeScript/JavaScript 扩展样例
- LSP (Language Server Protocol) 样例
- 核心配置文件

**移除内容**:
- `webview-sample` (20MB) - WebView 扩展样例
- 其他非核心语言样例和高级功能样例

## 📋 常用操作命令

### 查看当前 Sparse Checkout 状态

```bash
# 进入 submodule 目录
cd vscode-extension-builder/source/vscode-docs

# 查看当前 sparse checkout 配置
git sparse-checkout list

# 查看 sparse checkout 状态
git sparse-checkout status
```

### 修改 Sparse Checkout 配置

```bash
# 添加新的目录/文件
git sparse-checkout set 新目录名/

# 移除目录/文件
git sparse-checkout disable 要移除的目录/

# 完全重新设置
git sparse-checkout set 目录1/ 目录2/ 文件.md

# 添加单个文件（注意需要前导斜杠）
git sparse-checkout set /README.md
```

### 初始化和禁用

```bash
# 初始化 sparse checkout（首次使用）
git sparse-checkout init --no-cone

# 禁用 sparse checkout（恢复完整检出）
git sparse-checkout disable

# 使用 cone 模式（更高效但功能有限）
git sparse-checkout init --cone
```

## 🛠️ 维护操作指南

### 场景1: 更新 Submodule

```bash
# 在项目根目录
cd /home/l8ng/.claude/skills

# 更新所有 submodule
git submodule update --remote

# 如果更新后 sparse checkout 配置丢失，重新配置
cd vscode-extension-builder/source/vscode-docs
git sparse-checkout init --no-cone
git sparse-checkout set api/ docs/ learn/ *.md
```

### 场景2: 添加新的保留内容

```bash
# 假设需要添加新的文档目录
cd vscode-extension-builder/source/vscode-docs

# 添加到现有配置
git sparse-checkout set api/ docs/ learn/ 新目录/ *.md

# 验证配置
git sparse-checkout list
```

### 场景3: 临时访问完整内容

```bash
# 临时禁用 sparse checkout 查看完整内容
cd vscode-extension-builder/source/vscode-docs
git sparse-checkout disable

# 完成操作后重新启用
git sparse-checkout init --no-cone
git sparse-checkout set api/ docs/ learn/ *.md
```

### 场景4: 完全恢复到优化前状态

```bash
# 方法1: 使用备份的配置
git submodule deinit -f vscode-extension-builder/source/vscode-docs
git submodule update --init --recursive vscode-extension-builder/source/vscode-docs

# 方法2: 手动重新克隆
cd vscode-extension-builder/source
rm -rf vscode-docs
git clone git@github.com:microsoft/vscode-docs.git
```

## ⚠️ 注意事项和最佳实践

### 重要提醒

1. **不要修改 .gitmodules 文件**: sparse checkout 是工作区级别的配置，不应该修改 .gitmodules
2. **备份配置**: 每次修改前先备份当前配置
3. **逐步操作**: 一次只修改一个目录，避免大规模变更
4. **测试验证**: 修改后要验证技能功能是否正常

### 常见问题解决

**问题1: Sparse checkout 不生效**
```bash
# 确保已初始化
git sparse-checkout init --no-cone

# 检查配置文件
cat .git/info/sparse-checkout

# 强制更新工作区
git read-tree -mu HEAD
```

**问题2: 修改配置后文件消失**
```bash
# 检查配置是否正确
git sparse-checkout list

# 重新设置配置
git sparse-checkout set 正确的目录配置
```

**问题3: Submodule 更新后配置丢失**
```bash
# 重新初始化 sparse checkout
git sparse-checkout init --no-cone
git sparse-checkout set 原来的配置
```

## 📊 配置文件位置

### 各 Submodule 的 Sparse Checkout 配置位置

1. **VSCode Docs**:
   - 路径: `.git/modules/vscode-extension-builder/source/vscode-docs/info/sparse-checkout`
   - 大小: 155MB (原 2.3GB)

2. **VSCode Extension Samples**:
   - 路径: `.git/modules/vscode-extension-builder/source/vscode-extension-samples/info/sparse-checkout`
   - 大小: 4.9MB (原 83MB)

3. **Slint Expert**:
   - 未配置 sparse checkout（保持完整）
   - 大小: 32MB

## 🔄 日常维护检查清单

### 每月检查项目

- [ ] 验证所有 submodule 状态正常: `git submodule status`
- [ ] 检查技能核心功能是否可正常访问
- [ ] 确认保留的目录大小是否合理
- [ ] 备份当前配置到新的备份文件

### 更新前的准备

1. 备份当前配置
2. 记录当前 sparse checkout 设置
3. 规划更新后的配置恢复步骤
4. 准备测试用例验证功能

## 📞 故障恢复

### 紧急恢复命令

```bash
# 完全重置所有 submodule
git submodule deinit -f .
git submodule update --init --recursive

# 恢复 sparse checkout 配置
cd vscode-extension-builder/source/vscode-docs
git sparse-checkout init --no-cone
git sparse-checkout set api/ docs/ learn/ *.md

cd ../vscode-extension-samples
git sparse-checkout init --no-cone
git sparse-checkout set helloworld-sample helloworld-minimal-sample getting-started-sample completions-sample code-actions-sample lsp-sample package.json README.md LICENSE
```

### 联系支持

如果遇到复杂问题，可以：
1. 查看 Git 官方文档: `git help sparse-checkout`
2. 检查备份文件: `optimization_backup.md`
3. 查看优化总结: `optimization_summary.md`

---

*最后更新: 2025-10-31*
*适用版本: Git 2.25+*
*维护者: Skills 项目维护团队*