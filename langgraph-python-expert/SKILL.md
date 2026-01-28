---
name: langgraph-python-expert
description: Expert guidance for LangGraph Python library. Build stateful, multi-actor applications with LLMs using nodes, edges, and state management. Use when working with LangGraph, building agent workflows, state machines, or complex multi-step LLM applications. Requires langgraph, langchain-core packages.
---

# LangGraph Python Expert

Comprehensive expert for building sophisticated stateful applications with LangGraph, focusing on production-ready workflows, state management, and agent orchestration.

## 📚 Official Source Documentation

This skill includes access to the official LangGraph source code through the `source/langgraph/` directory (managed as git submodule with sparse-checkout), which contains:

- **Core Libraries**: `libs/langgraph/`, `libs/prebuilt/`, `libs/checkpoint*/`
- **Official Examples**: `examples/` - Up-to-date examples and tutorials
- **Complete Documentation**: `docs/docs/` - Latest documentation and API references

### Source Structure (66MB with sparse-checkout)

```
source/langgraph/
├── libs/
│   ├── langgraph/          # Core StateGraph, nodes, edges
│   ├── prebuilt/           # create_react_agent, ToolNode
│   ├── checkpoint/         # Base checkpoint classes
│   ├── checkpoint-sqlite/  # SQLite persistence
│   └── checkpoint-postgres/# PostgreSQL persistence
├── examples/               # Official examples and tutorials
├── docs/docs/              # Documentation (concepts, how-tos, reference)
├── README.md               # Project overview
├── CLAUDE.md               # Claude Code instructions
└── AGENTS.md               # Agent development guide
```

### Updating Source Code
```bash
cd source/langgraph
git pull origin main
```

For detailed structure, see [SOURCE_STRUCTURE.md](SOURCE_STRUCTURE.md).

## Quick Start

### Installation
```bash
pip install langgraph langchain-core langchain-openai
```

### Basic Concepts

**StateGraph**: The core component for building workflows with state persistence
**Nodes**: Functions that process the state and return updates
**Edges**: Define the flow between nodes (conditional or direct)
**State**: TypedDict that holds conversation/application state
**Persistence**: Checkpointing for memory and conversation history

## Core Components

### 1. State Definition
```python
from typing import TypedDict, List, Optional
from langchain_core.messages import BaseMessage

class AgentState(TypedDict):
    messages: List[BaseMessage]
    current_user: Optional[str]
    step_count: int
    requires_action: bool
```

### 2. Node Functions
```python
from langchain_core.messages import HumanMessage, AIMessage

def llm_node(state: AgentState) -> AgentState:
    """Process messages with LLM and return updated state"""
    messages = state["messages"]
    response = llm.invoke(messages)
    return {
        "messages": messages + [response],
        "step_count": state["step_count"] + 1
    }

def router_node(state: AgentState) -> str:
    """Decide next node based on state"""
    last_message = state["messages"][-1]
    if "tool_call" in last_message.additional_kwargs:
        return "tool_executor"
    return "end"
```

### 3. Graph Construction
```python
from langgraph.graph import StateGraph, END
from langgraph.checkpoint.memory import MemorySaver

# Create graph
workflow = StateGraph(AgentState)

# Add nodes
workflow.add_node("agent", agent_node)
workflow.add_node("tool_executor", tool_node)
workflow.add_node("router", router_node)

# Add edges
workflow.set_entry_point("agent")
workflow.add_conditional_edges(
    "agent",
    router_node,
    {
        "tool_executor": "tool_executor",
        "end": END
    }
)
workflow.add_edge("tool_executor", "agent")

# Memory
memory = MemorySaver()
app = workflow.compile(checkpointer=memory)
```

### 4. Running the Graph
```python
from langchain_core.messages import HumanMessage

# Run with thread_id for conversation memory
config = {"configurable": {"thread_id": "conversation-1"}}
response = app.invoke(
    {"messages": [HumanMessage(content="Hello!")]},
    config=config
)
```

### 5. Prebuilt Agents
```python
from langgraph.prebuilt import create_react_agent
from langchain_openai import ChatOpenAI

tools = [search_tool, calculator]
llm = ChatOpenAI(model="gpt-4")
agent = create_react_agent(llm, tools)

response = agent.invoke(
    {"messages": [HumanMessage(content="What's 2+2?")]}
)
```

## Testing and Debugging

### Testing Individual Nodes
```python
def test_llm_node():
    state = {
        "messages": [HumanMessage(content="Test")],
        "step_count": 0
    }
    result = llm_node(state)
    assert len(result["messages"]) == 2
    assert result["step_count"] == 1
```

### Graph Visualization
```python
from IPython.display import Image, display

try:
    display(Image(app.get_graph().draw_mermaid_png()))
except Exception:
    pass  # Requires graphviz
```

### Debug Mode
```python
from langgraph.types import interrupt

def debug_node(state):
    result = process(state)
    interrupt({"debug": result})  # Pause execution
    return result
```

## Troubleshooting

**Common Issues:**
- **State not updating**: Ensure nodes return state updates, not mutate state directly
- **Memory not persisting**: Check thread_id is consistent across invocations
- **Conditional edges not working**: Verify router function returns exact node names
- **Checkpoint errors**: Ensure checkpoint saver is initialized before compile()

**Debug Techniques:**
- Use `print()` statements in nodes to trace state
- Check state type hints match actual data
- Verify tool schemas are properly defined
- Test nodes individually before integrating

## Requirements

### Essential Packages
```bash
# Core
pip install langgraph langchain-core

# LLM Integrations
pip install langchain-openai    # OpenAI
pip install langchain-anthropic # Anthropic

# Optional Checkpointers
pip install langgraph-checkpoint-sqlite
pip install langgraph-checkpoint-postgres
```

### Optional Dependencies
```bash
# Visualization
pip install graphviz matplotlib

# Async Support
pip install aiohttp

# Development
pip install pytest pytest-asyncio
```

## Quick Reference

### Essential APIs
- `StateGraph(state_schema)` - Create graph with typed state
- `add_node(name, func)` - Register node function
- `set_entry_point(node)` - Set starting node
- `add_edge(from, to)` - Add direct connection
- `add_conditional_edges(node, router, mapping)` - Conditional routing
- `compile(checkpointer=...)` - Build executable graph
- `invoke(state, config)` - Run graph synchronously
- `stream(state, config)` - Stream outputs
- `astream(state, config)` - Async streaming

### Common Patterns
- **Agent Loop**: `agent → router → [tools/end] → agent`
- **Router**: Return string key from conditional edge mapping
- **State Updates**: Return partial dict from node functions
- **Memory**: Use thread_id in config for persistence

### Graph Methods
```python
# Execution
app.invoke(state, config)
app.stream(state, config, stream_mode="values")
app.batch([state1, state2], config)

# Inspection
app.get_graph()
app.get_graph().print_ascii()
```

> See [references/ADVANCED.md](references/ADVANCED.md) for advanced patterns including human-in-the-loop, async operations, and multi-agent systems.
> See [references/DEPLOYMENT.md](references/DEPLOYMENT.md) for production deployment, monitoring, and optimization strategies.
