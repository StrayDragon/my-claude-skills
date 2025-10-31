# LangGraph Source Documentation

这个目录包含 LangGraph 官方源代码和文档，通过 git submodule 管理以保持最新状态。

## 目录结构

```
source/
├── README.md                 # 本文件
├── langgraph/               # LangGraph 官方仓库 (git submodule)
│   ├── libs/                # 核心库代码
│   │   ├── langgraph/       # 主要 langgraph 库
│   │   └── langgraph-openai # OpenAI 集成
│   ├── examples/            # 官方示例
│   │   ├── basic/          # 基础示例
│   │   ├── agents/         # 代理示例
│   │   └── advanced/       # 高级示例
│   ├── docs/               # 官方文档
│   └── tests/              # 测试代码
└── scripts/                # 快速访问脚本
    ├── setup.sh            # 初始化脚本
    ├── update.sh           # 更新脚本
    └── explore.sh          # 探索脚本
```

## 使用说明

### 初始化
```bash
cd source
./scripts/setup.sh
```

### 更新到最新版本
```bash
./scripts/update.sh
```

### 快速探索
```bash
./scripts/explore.sh
```

## 主要组件

### 核心 API 文件
- `langgraph/libs/langgraph/src/langgraph/graph/` - 图构建相关 API
- `langgraph/libs/langgraph/src/langgraph/checkpoint/` - 检查点存储
- `langgraph/libs/langgraph/src/langgraph/prebuilt/` - 预构建组件

### 示例代码
- `langgraph/examples/basic/` - 基础用法示例
- `langgraph/examples/agents/` - 代理模式示例
- `langgraph/examples/advanced/` - 高级功能示例

### 测试代码
- `langgraph/tests/` - 完整的测试套件，展示各种使用模式

## 版本信息

当前跟踪的是 LangGraph 官方主分支的最新代码，确保获取最新的功能和修复。

## 注意事项

1. 这是一个 git submodule，需要单独初始化和更新
2. 大小可能较大，使用 git sparse-checkout 来精简下载
3. 官方代码会定期更新，建议定期运行 update.sh