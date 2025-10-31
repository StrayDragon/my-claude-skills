# LangGraph Quick Reference

## Core Components

### State Definition
```python
from typing import TypedDict, List, Optional
from langchain_core.messages import BaseMessage

class AgentState(TypedDict):
    messages: List[BaseMessage]
    current_step: str
    metadata: dict
```

### Node Function
```python
def node_function(state: AgentState) -> AgentState:
    """Process state and return updates"""
    # Your logic here
    return {
        "messages": state["messages"] + [new_message],
        "current_step": "next_step"
    }
```

### Graph Construction
```python
from langgraph.graph import StateGraph, START, END

workflow = StateGraph(AgentState)
workflow.add_node("node1", node_function)
workflow.add_edge("node1", "node2")
workflow.add_conditional_edges("node1", router_function)
workflow.set_entry_point("node1")
app = workflow.compile()
```

### State Persistence
```python
from langgraph.checkpoint.memory import MemorySaver
from langgraph.checkpoint.postgres import PostgresSaver

# Memory persistence
memory = MemorySaver()
app = workflow.compile(checkpointer=memory)

# PostgreSQL persistence
pg_saver = PostgresSaver.from_conn_string(db_url)
app = workflow.compile(checkpointer=pg_saver)
```

## Common Patterns

### 1. Basic Chat Agent
```python
def chat_agent(state):
    messages = state["messages"]
    response = llm.invoke(messages)
    return {"messages": messages + [response]}
```

### 2. Router Function
```python
def router(state):
    last_message = state["messages"][-1]
    if "tool_call" in last_message.additional_kwargs:
        return "tool_node"
    return END
```

### 3. Tool Execution
```python
def tool_node(state):
    last_message = state["messages"][-1]
    tool_results = []

    for tool_call in last_message.tool_calls:
        tool = tools[tool_call["name"]]
        result = tool.invoke(tool_call["args"])
        tool_results.append(ToolMessage(
            tool_call_id=tool_call["id"],
            content=str(result)
        ))

    return {"messages": state["messages"] + tool_results}
```

### 4. Human-in-the-Loop
```python
workflow.compile(
    checkpointer=memory,
    interrupt_before=["human_approval"],
    interrupt_after=["critical_action"]
)
```

## State Types

### MessagesState (Built-in)
```python
from langgraph.graph import MessagesState

class MyState(MessagesState):
    # Automatically includes 'messages' field
    custom_field: str
```

### Custom TypedDict
```python
from typing import TypedDict, List, Dict, Any

class CustomState(TypedDict):
    input_data: Dict[str, Any]
    intermediate_results: List[str]
    final_output: str
    error_count: int
```

## Edge Types

### Direct Edge
```python
workflow.add_edge("node1", "node2")
```

### Conditional Edge
```python
def condition(state):
    if state["needs_more_info"]:
        return "research"
    return "generate"

workflow.add_conditional_edges(
    "node1",
    condition,
    {
        "research": "research_node",
        "generate": "generate_node"
    }
)
```

### Entry Point
```python
workflow.set_entry_point("first_node")
# OR
workflow.add_edge(START, "first_node")
```

### End Point
```python
workflow.add_edge("last_node", END)
```

## Checkpointing

### Memory Checkpoint
```python
from langgraph.checkpoint.memory import MemorySaver

memory = MemorySaver()
app = workflow.compile(checkpointer=memory)

# Usage
config = {"configurable": {"thread_id": "conversation-1"}}
result = app.invoke(initial_state, config)
```

### SQLite Checkpoint
```python
from langgraph.checkpoint.sqlite import SqliteSaver

sqlite_saver = SqliteSaver.from_conn_string(":memory:")
app = workflow.compile(checkpointer=sqlite_saver)
```

### PostgreSQL Checkpoint
```python
from langgraph.checkpoint.postgres import PostgresSaver

pg_saver = PostgresSaver.from_conn_string(
    "postgresql://user:pass@localhost/langgraph"
)
app = workflow.compile(checkpointer=pg_saver)
```

## Error Handling

### Try-Catch in Nodes
```python
def safe_node(state):
    try:
        result = risky_operation(state)
        return {"result": result}
    except Exception as e:
        return {
            "error": str(e),
            "messages": state["messages"] + [
                AIMessage(content=f"Error: {e}")
            ]
        }
```

### Retry Pattern
```python
def retry_node(state):
    retry_count = state.get("retry_count", 0)
    max_retries = state.get("max_retries", 3)

    try:
        result = operation(state)
        return {
            "result": result,
            "retry_count": 0  # Reset on success
        }
    except Exception as e:
        if retry_count >= max_retries:
            return {"error": f"Max retries exceeded: {e}"}

        return {
            "retry_count": retry_count + 1,
            "last_error": str(e)
        }
```

## Prebuilt Components

### React Agent
```python
from langgraph.prebuilt import create_react_agent

agent = create_react_agent(llm, tools)
result = agent.invoke({"messages": [HumanMessage(content="query")]})
```

### Tool Node
```python
from langgraph.prebuilt import ToolNode

tool_node = ToolNode(tools)
workflow.add_node("tools", tool_node)
```

## Streaming

### Stream Values
```python
for chunk in app.stream(initial_state, config):
    print(chunk)
```

### Stream Events
```python
async for event in app.astream_events(initial_state, config):
    if event["event"] == "on_chat_model_stream":
        print(event["data"]["chunk"].content)
```

## Common Imports

```python
# Core imports
from langgraph.graph import StateGraph, MessagesState, START, END
from langgraph.checkpoint.memory import MemorySaver
from langgraph.prebuilt import create_react_agent, ToolNode

# Message types
from langchain_core.messages import (
    HumanMessage, AIMessage, SystemMessage,
    ToolMessage, BaseMessage
)

# Type definitions
from typing import TypedDict, List, Dict, Any, Optional
```

## Configuration Patterns

### Thread Configuration
```python
config = {
    "configurable": {
        "thread_id": "unique-conversation-id",
        "checkpoint_ns": "namespace"  # Optional
    }
}
```

### LLM Configuration
```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    model="gpt-4",
    temperature=0.7,
    max_tokens=1000
)
```

## Debugging

### Enable Debug Mode
```python
import langgraph
langgraph.debug = True
```

### Print State Transitions
```python
def debug_node(state):
    print(f"Input state: {state}")
    result = your_node_logic(state)
    print(f"Output state: {result}")
    return result
```

### Graph Visualization
```python
try:
    from IPython.display import Image, display
    display(Image(app.get_graph().draw_mermaid_png()))
except:
    print("Visualization not available")
```

## Performance Tips

1. **Minimize State Size**: Store only necessary data in state
2. **Use Streaming**: For long-running operations
3. **Optimize Checkpoints**: Regular cleanup of old checkpoints
4. **Batch Operations**: When possible, batch multiple operations
5. **Async/Await**: Use for I/O-bound operations
6. **Caching**: Cache expensive operations

## Troubleshooting

### Common Issues

**State too large**: Move large data to external storage
**Memory leaks**: Clean up unused state data
**Slow execution**: Check for inefficient operations
**Checkpoint errors**: Verify database connectivity

### Debug Commands

```python
# Check graph structure
print(app.get_graph().draw_mermaid())

# Inspect state
print(app.get_state(config))

# Get graph history
for state in app.get_state_history(config):
    print(state.values)
```

## Best Practices

1. **Keep nodes focused** on single responsibilities
2. **Use type hints** for better code clarity
3. **Implement proper error handling**
4. **Design for testability**
5. **Use descriptive names** for nodes and functions
6. **Document complex routing logic**
7. **Test individual nodes** in isolation
8. **Monitor checkpoint sizes** in production