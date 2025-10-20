# Slint 官方示例指南

本目录下的所有示例都直接引用官方 Slint 仓库中的最新示例代码。

## 官方示例目录结构

### @source/examples/
官方仓库包含的完整示例集合：

- **printerdemo** - 完整的打印机控制应用程序
  - 复杂的 UI 组件和状态管理
  - 数据绑定和动态更新
  - 多窗口和对话框

- **slide_puzzle** - 滑块拼图游戏
  - 游戏逻辑和用户交互
  - 动画和过渡效果
  - 状态管理和游戏控制

- **energy monitor** - 能源监控仪表盘
  - 数据可视化
  - 实时数据更新
  - 图表和仪表组件

- **gallery** - UI 组件展示
  - 各种 UI 组件的完整展示
  - 样式和主题示例
  - 布局和动画效果

- **todo** - 待办事项应用
  - 基础的 CRUD 操作
  - 列表管理和状态同步
  - 用户输入处理

- **memory** - 记忆游戏
  - 游戏状态管理
  - 计时器和得分系统
  - 卡片翻转动画

## 如何使用官方示例

### 1. 直接运行官方示例
```bash
# 进入官方示例目录
cd @source/examples/gallery

# 运行示例
cargo run
```

### 2. 参考官方示例进行学习
每个官方示例都是学习特定功能的最佳资源：

- **基础学习**: 从 `todo` 和 `memory` 开始
- **UI 组件**: 参考 `gallery` 了解所有可用组件
- **复杂应用**: 学习 `printerdemo` 的架构设计
- **游戏开发**: 研究 `slide_puzzle` 的游戏逻辑
- **数据可视化**: 参考 `energy monitor` 的数据处理

### 3. 在自己的项目中应用
参考官方示例的代码模式，在我们的 templates 中应用：

- **基础应用模板** (`templates/basic-app/`) - 参考 `todo` 示例
- **组件库模板** (`templates/component-library/`) - 参考 `gallery` 组件
- **跨平台模板** (`templates/cross-platform/`) - 结合多个示例的 WebAssembly 支持

## 推荐学习路径

1. **入门**: `todo` → `memory`
2. **组件学习**: `gallery`
3. **复杂应用**: `printerdemo`
4. **专业特性**: `slide_puzzle` + `energy monitor`

## 注意事项

- 所有示例都是官方维护的最新版本
- 代码遵循 Rust 和 Slint 的最佳实践
- 包含完整的构建配置和依赖管理
- 可以直接运行和学习，无需额外配置