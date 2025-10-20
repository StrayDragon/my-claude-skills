# Slint 官方文档指南

本目录下的所有文档都直接引用官方 Slint 仓库中的最新文档。

## 官方文档目录结构

### @source/docs/
官方仓库包含的完整文档集合：

### 语言和教程
- **tutorial/** - 官方教程
  - 从零开始学习 Slint
  - 循序渐进的课程设计
  - 实践项目示例

- **language/** - 语言参考
  - 语法完整说明
  - 内置组件和属性
  - 表达式和绑定系统

- **builtin-elements/** - 内置组件文档
  - 所有可用 UI 组件的详细说明
  - 属性、方法和事件
  - 使用示例和最佳实践

### 高级主题
- **integrations/** - 集成指南
  - Rust 集成详细说明
  - C++ 集成文档
  - WebAssembly 部署指南

- **cookbook/** - 实用手册
  - 常见问题解决方案
  - 最佳实践集合
  - 性能优化技巧

- **howto/** - 操作指南
  - 具体功能的实现方法
  - 步骤详细的教程
  - 代码示例和解释

## 如何使用官方文档

### 1. 学习路径建议
```bash
# 初学者路径
@source/docs/tutorial/ → @source/docs/language/ → @source/docs/cookbook/

# 进阶开发者路径
@source/docs/language/ → @source/docs/integrations/ → @source/docs/howto/
```

### 2. 快速参考
- **语法查询**: `@source/docs/language/syntax.md`
- **组件参考**: `@source/docs/builtin-elements/`
- **集成问题**: `@source/docs/integrations/rust/`
- **最佳实践**: `@source/docs/coookbook/`

### 3. 与 Skill 模板结合使用
在我们的模板开发中，可以参考：

- **基础应用开发**: 参考 `@source/docs/tutorial/` 和 `@source/docs/language/`
- **组件设计**: 参考 `@source/docs/builtin-elements/` 和 `@source/docs/cookbook/`
- **高级功能**: 参考 `@source/docs/integrations/` 和 `@source/docs/howto/`

## 推荐文档阅读顺序

### 入门阶段
1. `@source/docs/tutorial/hello_world.md`
2. `@source/docs/tutorial/components.md`
3. `@source/docs/tutorial/layouts.md`
4. `@source/docs/tutorial/callbacks.md`

### 进阶阶段
1. `@source/docs/language/bindings.md`
2. `@source/docs/language/properties.md`
3. `@source/docs/cookbook/state-management.md`
4. `@source/docs/cookbook/animations.md`

### 专家阶段
1. `@source/docs/integrations/rust/complex-data.md`
2. `@source/docs/howto/performance.md`
3. `@source/docs/cookbook/advanced-styling.md`

## 本地文档导航

为了方便使用，我们在本地提供了一些导航文件：

- **本文件** (`docs/README.md`) - 文档总览和使用指南
- **快速引用** - 常用功能的快速查找指南
- **最佳实践** - 基于官方文档的实践总结

## 注意事项

- 所有文档都是官方维护的最新版本
- 文档与当前 Slint 版本完全同步
- 包含最新的 API 变更和功能更新
- 可以通过官方仓库获取最新的文档修订