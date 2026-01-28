## Advanced Patterns

### 1. Multi-Agent Collaboration
```python
from langgraph.graph import StateGraph, MessagesState
from langgraph.prebuilt import create_react_agent

class MultiAgentState(MessagesState):
    researcher_notes: str
    writer_content: str
    reviewer_feedback: List[str]

def researcher_node(state: MultiAgentState) -> MultiAgentState:
    """Research agent that gathers information"""
    researcher_agent = create_react_agent(llm, research_tools)
    result = researcher_agent.invoke({
        "messages": state["messages"][-2:]  # Last two messages
    })

    return {
        "researcher_notes": result["messages"][-1].content,
        "messages": state["messages"] + result["messages"]
    }

def writer_node(state: MultiAgentState) -> MultiAgentState:
    """Writer agent that creates content based on research"""
    writer_agent = create_react_agent(llm, writing_tools)
    prompt = f"Research notes: {state['researcher_notes']}"

    result = writer_agent.invoke({
        "messages": [HumanMessage(content=prompt)]
    })

    return {
        "writer_content": result["messages"][-1].content,
        "messages": state["messages"] + result["messages"]
    }
```

### 2. Dynamic Tool Selection
```python
from typing import Dict, Any
from langchain_core.tools import BaseTool

class DynamicToolNode:
    def __init__(self, tool_registry: Dict[str, BaseTool]):
        self.tool_registry = tool_registry

    def __call__(self, state: AgentState) -> AgentState:
        last_message = state["messages"][-1]

        if not last_message.tool_calls:
            return state

        # Dynamically select tools based on context
        selected_tools = self.select_tools_by_context(state)

        # Execute tool calls
        tool_messages = []
        for tool_call in last_message.tool_calls:
            if tool_call["name"] in selected_tools:
                tool = selected_tools[tool_call["name"]]
                result = tool.invoke(tool_call["args"])
                tool_messages.append(
                    ToolMessage(
                        tool_call_id=tool_call["id"],
                        content=str(result)
                    )
                )

        return {
            "messages": state["messages"] + tool_messages
        }

    def select_tools_by_context(self, state: AgentState) -> Dict[str, BaseTool]:
        """Intelligently select tools based on conversation context"""
        context = " ".join([msg.content for msg in state["messages"][-5:]])

        available_tools = {}
        if "code" in context.lower():
            available_tools.update({"code_executor": code_tool})
        if "search" in context.lower():
            available_tools.update({"web_search": search_tool})
        if "math" in context.lower():
            available_tools.update({"calculator": math_tool})

        return available_tools
```

### 3. State Persistence and Recovery
```python
from langgraph.checkpoint.sqlite import SqliteSaver
from langgraph.checkpoint.postgres import PostgresSaver

# Production-ready persistence
def create_production_app():
    # Use PostgreSQL for production
    connection_string = "postgresql://user:pass@localhost/langgraph"
    checkpointer = PostgresSaver.from_conn_string(connection_string)

    # Build workflow
    workflow = StateGraph(AgentState)
    # ... add nodes and edges

    # Compile with persistence
    app = workflow.compile(checkpointer=checkpointer)
    return app

# Thread-based conversation management
def manage_conversation(app, thread_id: str):
    """Manage persistent conversations across sessions"""
    config = {"configurable": {"thread_id": thread_id}}

    # Continue existing conversation
    result = app.invoke({
        "messages": [HumanMessage(content="Continue our discussion")]
    }, config)

    return result
```

### 4. Error Handling and Retry Logic
```python
from typing import Union
from langgraph.graph import StateGraph
import time

class RobustAgentState(TypedDict):
    messages: List[BaseMessage]
    retry_count: int
    max_retries: int
    error_history: List[str]

def error_handling_node(state: RobustAgentState) -> Union[RobustAgentState, str]:
    """Node with built-in error handling and retry logic"""
    try:
        # Attempt the primary operation
        result = perform_operation(state)

        # Reset retry count on success
        return {
            **result,
            "retry_count": 0,
            "error_history": []
        }

    except Exception as e:
        error_msg = str(e)
        new_retry_count = state["retry_count"] + 1

        if new_retry_count >= state["max_retries"]:
            return "error_handler"  # Route to error handling

        # Add delay for exponential backoff
        time.sleep(2 ** new_retry_count)

        return {
            "retry_count": new_retry_count,
            "error_history": state["error_history"] + [error_msg]
        }

def fallback_node(state: RobustAgentState) -> RobustAgentState:
    """Fallback strategy when primary operation fails"""
    last_error = state["error_history"][-1] if state["error_history"] else "Unknown error"

    fallback_message = AIMessage(
        content=f"I encountered an error: {last_error}. "
                f"Let me try a different approach."
    )

    return {
        "messages": state["messages"] + [fallback_message],
        "retry_count": 0
    }
```

