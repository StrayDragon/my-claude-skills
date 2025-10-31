# LangGraph Examples

## Example 1: Simple Agent Workflow

```python
from typing import TypedDict, List
from langchain_core.messages import BaseMessage
from langgraph.graph import StateGraph, END
from langgraph.checkpoint.memory import MemorySaver
from langchain_openai import ChatOpenAI

# State definition
class SimpleAgentState(TypedDict):
    messages: List[BaseMessage]
    user_name: str

# Initialize LLM
llm = ChatOpenAI(model="gpt-4")

def chat_node(state: SimpleAgentState) -> SimpleAgentState:
    """Simple chat node that responds to user messages"""
    messages = state["messages"]

    # Add system message if not present
    if len(messages) == 1 and messages[0].type == "human":
        system_msg = SystemMessage(content=f"Hello {state['user_name']}! How can I help you today?")
        messages = [system_msg] + messages

    response = llm.invoke(messages)
    return {"messages": messages + [response]}

def should_continue(state: SimpleAgentState) -> str:
    """Determine if conversation should continue"""
    last_message = state["messages"][-1]
    if "goodbye" in last_message.content.lower():
        return END
    return "continue"

# Build workflow
workflow = StateGraph(SimpleAgentState)
workflow.add_node("chat", chat_node)
workflow.set_entry_point("chat")
workflow.add_conditional_edges("chat", should_continue)

# Compile with memory
memory = MemorySaver()
app = workflow.compile(checkpointer=memory)

# Usage
config = {"configurable": {"thread_id": "conversation-1"}}
result = app.invoke({
    "messages": [HumanMessage(content="Hello!")],
    "user_name": "Alice"
}, config)
```

## Example 2: RAG (Retrieval-Augmented Generation)

```python
from typing import TypedDict, List
from langchain_core.documents import Document
from langchain_core.messages import HumanMessage, AIMessage
from langchain_openai import ChatOpenAI, OpenAIEmbeddings
from langchain_community.vectorstores import FAISS
from langchain_text_splitters import RecursiveCharacterTextSplitter

class RAGState(TypedDict):
    question: str
    context: List[Document]
    answer: str
    chat_history: List[dict]

def create_rag_workflow():
    llm = ChatOpenAI(model="gpt-4")
    embeddings = OpenAIEmbeddings()

    # Sample documents (replace with your actual data)
    documents = [
        Document(page_content="Python is a high-level programming language...", metadata={"source": "doc1"}),
        Document(page_content="LangGraph is a library for building stateful agents...", metadata={"source": "doc2"}),
    ]

    # Create vector store
    text_splitter = RecursiveCharacterTextSplitter(chunk_size=1000, chunk_overlap=200)
    texts = text_splitter.split_documents(documents)
    vectorstore = FAISS.from_documents(texts, embeddings)

    def retrieve_node(state: RAGState) -> RAGState:
        """Retrieve relevant documents for the question"""
        docs = vectorstore.similarity_search(state["question"], k=3)
        return {"context": docs}

    def generate_node(state: RAGState) -> RAGState:
        """Generate answer based on retrieved context"""
        context_text = "\n\n".join([doc.page_content for doc in state["context"]])

        prompt = f"""
        Based on the following context, answer the user's question.

        Context:
        {context_text}

        Question: {state['question']}

        Provide a comprehensive answer based on the context provided.
        """

        response = llm.invoke([HumanMessage(content=prompt)])
        return {"answer": response.content}

    # Build workflow
    workflow = StateGraph(RAGState)
    workflow.add_node("retrieve", retrieve_node)
    workflow.add_node("generate", generate_node)

    workflow.set_entry_point("retrieve")
    workflow.add_edge("retrieve", "generate")
    workflow.add_edge("generate", END)

    return workflow.compile()

# Usage
app = create_rag_workflow()
result = app.invoke({
    "question": "What is LangGraph?",
    "context": [],
    "answer": "",
    "chat_history": []
})
```

## Example 3: Multi-Agent Collaboration

```python
from typing import TypedDict, List
from langchain_core.messages import BaseMessage, HumanMessage, AIMessage
from langchain_openai import ChatOpenAI
from langgraph.graph import StateGraph, END
from langgraph.prebuilt import create_react_agent
from langchain_core.tools import tool

class MultiAgentState(TypedDict):
    messages: List[BaseMessage]
    research_findings: str
    draft_content: str
    final_output: str
    review_comments: List[str]

# Define tools
@tool
def search_web(query: str) -> str:
    """Search the web for information"""
    # Implement your web search logic here
    return f"Search results for: {query}"

@tool
def save_document(content: str, filename: str) -> str:
    """Save content to a file"""
    # Implement your document saving logic here
    return f"Saved content to {filename}"

def create_multi_agent_workflow():
    # Initialize agents
    llm = ChatOpenAI(model="gpt-4")

    researcher_agent = create_react_agent(llm, [search_web])
    writer_agent = create_react_agent(llm, [save_document])
    reviewer_agent = create_react_agent(llm, [])

    def researcher_node(state: MultiAgentState) -> MultiAgentState:
        """Research agent that gathers information"""
        last_message = state["messages"][-1]

        result = researcher_agent.invoke({
            "messages": [HumanMessage(content=f"Research: {last_message.content}")]
        })

        return {
            "research_findings": result["messages"][-1].content,
            "messages": state["messages"] + [
                AIMessage(content=f"Research completed: {result['messages'][-1].content}")
            ]
        }

    def writer_node(state: MultiAgentState) -> MultiAgentState:
        """Writer agent that creates content"""
        prompt = f"""
        Based on these research findings, create a well-structured document:

        Research Findings:
        {state['research_findings']}

        Create comprehensive content.
        """

        result = writer_agent.invoke({
            "messages": [HumanMessage(content=prompt)]
        })

        return {
            "draft_content": result["messages"][-1].content,
            "messages": state["messages"] + [
                AIMessage(content=f"Draft created: {result['messages'][-1].content}")
            ]
        }

    def reviewer_node(state: MultiAgentState) -> MultiAgentState:
        """Reviewer agent that evaluates the content"""
        prompt = f"""
        Review this content for quality, accuracy, and completeness:

        {state['draft_content']}

        Provide specific feedback and suggestions for improvement.
        """

        result = reviewer_agent.invoke({
            "messages": [HumanMessage(content=prompt)]
        })

        feedback = result["messages"][-1].content

        return {
            "review_comments": [feedback],
            "messages": state["messages"] + [
                AIMessage(content=f"Review completed: {feedback}")
            ]
        }

    def finalizer_node(state: MultiAgentState) -> MultiAgentState:
        """Finalize the content based on all feedback"""
        prompt = f"""
        Create the final version incorporating this feedback:

        Original Draft:
        {state['draft_content']}

        Review Comments:
        {state['review_comments']}

        Produce the polished final content.
        """

        result = llm.invoke([HumanMessage(content=prompt)])

        return {
            "final_output": result.content,
            "messages": state["messages"] + [
                AIMessage(content=f"Final output: {result.content}")
            ]
        }

    def routing_logic(state: MultiAgentState) -> str:
        """Determine the next step in the workflow"""
        if not state.get("research_findings"):
            return "researcher"
        elif not state.get("draft_content"):
            return "writer"
        elif not state.get("review_comments"):
            return "reviewer"
        else:
            return "finalizer"

    # Build workflow
    workflow = StateGraph(MultiAgentState)
    workflow.add_node("researcher", researcher_node)
    workflow.add_node("writer", writer_node)
    workflow.add_node("reviewer", reviewer_node)
    workflow.add_node("finalizer", finalizer_node)

    workflow.add_conditional_edges(
        START,
        routing_logic,
        {
            "researcher": "researcher",
            "writer": "writer",
            "reviewer": "reviewer",
            "finalizer": "finalizer"
        }
    )

    workflow.add_edge("researcher", START)  # Loop back to routing
    workflow.add_edge("writer", START)      # Loop back to routing
    workflow.add_edge("reviewer", START)    # Loop back to routing
    workflow.add_edge("finalizer", END)

    return workflow.compile()

# Usage
app = create_multi_agent_workflow()
result = app.invoke({
    "messages": [HumanMessage(content="Create a comprehensive guide about LangGraph")],
    "research_findings": "",
    "draft_content": "",
    "final_output": "",
    "review_comments": []
})
```

## Example 4: Workflow with Human-in-the-Loop

```python
from typing import TypedDict, List
from langchain_core.messages import BaseMessage, HumanMessage, AIMessage
from langgraph.graph import StateGraph, START, END
from langgraph.checkpoint.memory import MemorySaver
import input  # For human input (in production, use a proper UI)

class HumanLoopState(TypedDict):
    messages: List[BaseMessage]
    current_task: str
    human_approval_required: bool
    approval_received: bool
    task_result: str

def create_human_loop_workflow():
    def task_planner(state: HumanLoopState) -> HumanLoopState:
        """Plan the next task and determine if approval is needed"""
        last_message = state["messages"][-1]

        # Simple logic: if task involves external actions, require approval
        needs_approval = any(keyword in last_message.content.lower()
                           for keyword in ["send", "delete", "modify", "execute"])

        task_description = f"Execute: {last_message.content}"

        return {
            "current_task": task_description,
            "human_approval_required": needs_approval,
            "approval_received": False,
            "task_result": ""
        }

    def human_approval_node(state: HumanLoopState) -> HumanLoopState:
        """Wait for human approval"""
        if not state["approval_received"]:
            print(f"\n⚠️  Task requires human approval:")
            print(f"Task: {state['current_task']}")

            while True:
                response = input("Do you approve this task? (yes/no): ").lower().strip()
                if response in ["yes", "y"]:
                    print("✅ Task approved!")
                    return {"approval_received": True}
                elif response in ["no", "n"]:
                    print("❌ Task rejected!")
                    return {
                        "approval_received": False,
                        "messages": state["messages"] + [
                            AIMessage(content="Task was rejected by human operator.")
                        ]
                    }
                else:
                    print("Please enter 'yes' or 'no'")

        return {}

    def task_executor(state: HumanLoopState) -> HumanLoopState:
        """Execute the approved task"""
        if state["approval_received"]:
            # Simulate task execution
            result = f"Successfully executed: {state['current_task']}"

            return {
                "task_result": result,
                "messages": state["messages"] + [
                    AIMessage(content=f"Task completed: {result}")
                ]
            }
        else:
            return {
                "messages": state["messages"] + [
                    AIMessage(content="Task was not executed due to lack of approval.")
                ]
            }

    def routing_logic(state: HumanLoopState) -> str:
        """Determine the next step"""
        if state["human_approval_required"] and not state["approval_received"]:
            return "human_approval"
        elif state["approval_received"] and not state["task_result"]:
            return "execute"
        else:
            return END

    # Build workflow
    workflow = StateGraph(HumanLoopState)
    workflow.add_node("planner", task_planner)
    workflow.add_node("human_approval", human_approval_node)
    workflow.add_node("execute", task_executor)

    workflow.set_entry_point("planner")
    workflow.add_conditional_edges("planner", routing_logic)
    workflow.add_edge("human_approval", "planner")
    workflow.add_edge("execute", END)

    # Add interrupts for human-in-the-loop
    memory = MemorySaver()
    app = workflow.compile(
        checkpointer=memory,
        interrupt_before=["human_approval"]
    )

    return app

# Usage
app = create_human_loop_workflow()

# Start a conversation
config = {"configurable": {"thread_id": "human-loop-demo"}}

result = app.invoke({
    "messages": [HumanMessage(content="Send an email to the team")],
    "current_task": "",
    "human_approval_required": False,
    "approval_received": False,
    "task_result": ""
}, config)

print("Initial result:", result["messages"][-1].content)

# If approval is needed, you can resume after human input
if result.get("human_approval_required"):
    print("\nWorkflow interrupted. Waiting for human approval...")

    # After getting approval, you can resume:
    # result = app.invoke(None, config)
```

## Example 5: Sequential Task Processor

```python
from typing import TypedDict, List, Dict, Any
from langchain_core.messages import HumanMessage, AIMessage
from langgraph.graph import StateGraph, START, END
import time
import random

class TaskProcessorState(TypedDict):
    tasks: List[Dict[str, Any]]
    current_index: int
    results: List[Dict[str, Any]]
    status: str
    errors: List[str]

def create_sequential_processor():
    def task_executor(state: TaskProcessorState) -> TaskProcessorState:
        """Execute the current task"""
        if state["current_index"] >= len(state["tasks"]):
            return {"status": "completed"}

        current_task = state["tasks"][state["current_index"]]
        task_id = current_task.get("id", f"task_{state['current_index']}")
        task_type = current_task.get("type", "default")

        try:
            # Simulate task execution
            print(f"Executing {task_type} task: {task_id}")
            time.sleep(1)  # Simulate processing time

            # Different execution logic based on task type
            if task_type == "data_processing":
                result = simulate_data_processing(current_task)
            elif task_type == "api_call":
                result = simulate_api_call(current_task)
            elif task_type == "calculation":
                result = simulate_calculation(current_task)
            else:
                result = simulate_default_task(current_task)

            return {
                "current_index": state["current_index"] + 1,
                "results": state["results"] + [{
                    "task_id": task_id,
                    "task_type": task_type,
                    "result": result,
                    "status": "completed",
                    "timestamp": time.time()
                }]
            }

        except Exception as e:
            error_msg = f"Error executing task {task_id}: {str(e)}"
            print(error_msg)

            return {
                "errors": state["errors"] + [error_msg],
                "current_index": state["current_index"] + 1,
                "results": state["results"] + [{
                    "task_id": task_id,
                    "task_type": task_type,
                    "result": None,
                    "status": "failed",
                    "error": str(e),
                    "timestamp": time.time()
                }]
            }

    def routing_logic(state: TaskProcessorState) -> str:
        """Determine if there are more tasks to process"""
        if state["current_index"] < len(state["tasks"]):
            return "continue"
        else:
            return "end"

    def final_summary(state: TaskProcessorState) -> TaskProcessorState:
        """Generate final summary of all tasks"""
        completed_tasks = [r for r in state["results"] if r["status"] == "completed"]
        failed_tasks = [r for r in state["results"] if r["status"] == "failed"]

        summary = f"""
        Task Processing Summary:
        - Total tasks: {len(state["tasks"])}
        - Completed: {len(completed_tasks)}
        - Failed: {len(failed_tasks)}
        - Errors: {len(state["errors"])}

        Completed Tasks:
        {[task["task_id"] for task in completed_tasks]}

        Failed Tasks:
        {[task["task_id"] for task in failed_tasks]}
        """

        return {
            "status": "completed",
            "messages": [AIMessage(content=summary)]
        }

    # Helper functions for task simulation
    def simulate_data_processing(task):
        data = task.get("data", {})
        processed = {k: v.upper() if isinstance(v, str) else v for k, v in data.items()}
        return processed

    def simulate_api_call(task):
        endpoint = task.get("endpoint", "https://api.example.com")
        # Simulate API response
        return {"status": "success", "data": f"Response from {endpoint}"}

    def simulate_calculation(task):
        operation = task.get("operation", "add")
        values = task.get("values", [1, 2, 3])

        if operation == "add":
            return sum(values)
        elif operation == "multiply":
            result = 1
            for v in values:
                result *= v
            return result
        else:
            return random.choice(values)

    def simulate_default_task(task):
        return {"message": f"Processed default task with data: {task.get('data', {})}"}

    # Build workflow
    workflow = StateGraph(TaskProcessorState)
    workflow.add_node("execute", task_executor)
    workflow.add_node("summary", final_summary)

    workflow.set_entry_point("execute")
    workflow.add_conditional_edges("execute", routing_logic, {
        "continue": "execute",
        "end": "summary"
    })
    workflow.add_edge("summary", END)

    return workflow.compile()

# Usage
app = create_sequential_processor()

# Define a list of tasks
tasks = [
    {"id": "task_1", "type": "data_processing", "data": {"name": "Alice", "city": "New York"}},
    {"id": "task_2", "type": "api_call", "endpoint": "https://api.weather.com"},
    {"id": "task_3", "type": "calculation", "operation": "add", "values": [10, 20, 30]},
    {"id": "task_4", "type": "calculation", "operation": "multiply", "values": [2, 3, 4]},
    {"id": "task_5", "type": "default", "data": {"action": "cleanup"}},
]

result = app.invoke({
    "tasks": tasks,
    "current_index": 0,
    "results": [],
    "status": "running",
    "errors": []
})

print("Final result:")
print(result["messages"][-1].content if result.get("messages") else "No messages")
print(f"Results: {len(result['results'])} tasks processed")
```

These examples demonstrate various LangGraph patterns and use cases. You can adapt them to your specific needs and build more complex workflows by combining these patterns.