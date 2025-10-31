# LangGraph Source Quick Access

快速访问 LangGraph 官方源代码的关键部分。

## 🚀 快速开始

```bash
# 初始化
cd source
./scripts/setup.sh

# 探索代码
./scripts/explore.sh

# 更新到最新版本
./scripts/update.sh
```

## 📂 核心文件位置

### 主要 API 类和函数

**图构建相关：**
```
langgraph/libs/langgraph/src/langgraph/graph/__init__.py
├── StateGraph - 主要的图构建类
├── MessageGraph - 消息图类
└── END, START - 常量定义
```

**检查点存储：**
```
langgraph/libs/langgraph/src/langgraph/checkpoint/
├── memory.py - 内存检查点
├── sqlite.py - SQLite 检查点
└── postgres.py - PostgreSQL 检查点
```

**预构建组件：**
```
langgraph/libs/langgraph/src/langgraph/prebuilt/
├── create_react_agent.py - React 代理创建器
├── tool_node.py - 工具执行节点
└── chat_agent_executor.py - 聊天代理执行器
```

### 示例代码

**基础示例：**
```
langgraph/examples/basic/
├── basic_chat.py - 基础聊天
├── human_in_the_loop.py - 人机交互
└── multi_agent.py - 多代理
```

**高级示例：**
```
langgraph/examples/agents/
├── agent_executor.py - 代理执行器
├── openai_functions_agent.py - OpenAI 函数代理
└── conversational_retrieval.py - 对话检索
```

## 🔍 代码导航技巧

### 1. 查找特定功能
```bash
# 查找 StateGraph 的实现
find langgraph -name "*.py" -exec grep -l "class StateGraph" {} \;

# 查找检查点相关的实现
find langgraph -name "*.py" -exec grep -l "class.*Checkpoint" {} \;
```

### 2. 查看最新变更
```bash
cd langgraph
git log --oneline -10  # 最近 10 个提交
git log --oneline --since="1 week ago"  # 最近一周的变更
```

### 3. 查找使用示例
```bash
# 在示例中查找特定 API 的使用
grep -r "StateGraph" langgraph/examples/ --include="*.py"
```

## 📚 重要文件说明

### 核心实现文件
1. **`libs/langgraph/src/langgraph/graph/graph.py`** - StateGraph 的主要实现
2. **`libs/langgraph/src/langgraph/graph/message_graph.py`** - MessageGraph 实现
3. **`libs/langgraph/src/langgraph/checkpoint/base.py`** - 检查点基类
4. **`libs/langgraph/src/langgraph/pregel/__init__.py`** - Pregel 算法实现

### 配置文件
1. **`pyproject.toml`** - 项目配置和依赖
2. **`README.md`** - 官方说明文档

## 🛠️ 开发和测试

### 运行测试
```bash
cd langgraph
# 运行所有测试
pytest

# 运行特定测试
pytest tests/test_graph.py

# 查看测试覆盖
pytest --cov=langgraph
```

### 运行示例
```bash
cd langgraph/examples/basic
python basic_chat.py
```

## 📖 学习路径建议

1. **初学者：**
   - 从 `examples/basic/` 开始
   - 阅读 `libs/langgraph/src/langgraph/graph/__init__.py`

2. **中级：**
   - 研究 `examples/agents/` 中的示例
   - 查看 `libs/langgraph/src/langgraph/prebuilt/` 组件

3. **高级：**
   - 阅读 `libs/langgraph/src/langgraph/pregel/` 实现
   - 研究检查点系统 `libs/langgraph/src/langgraph/checkpoint/`

## 🔧 自定义和扩展

### 添加新的检查点存储
参考 `libs/langgraph/src/langgraph/checkpoint/` 中的现有实现

### 创建自定义节点
查看 `examples/` 中的自定义节点示例

### 扩展预构建组件
参考 `libs/langgraph/src/langgraph/prebuilt/` 中的组件

## 📞 获取帮助

- **官方文档：** 查看 `docs/` 目录
- **示例代码：** `examples/` 目录
- **测试用例：** `tests/` 目录中的测试用例展示了各种使用模式
- **GitHub Issues：** [LangGraph Issues](https://github.com/langchain-ai/langgraph/issues)