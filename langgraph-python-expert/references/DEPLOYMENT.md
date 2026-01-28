## Production Deployment

### 1. Environment Setup
```python
import os
from langgraph.graph import StateGraph
from langgraph.checkpoint.postgres import PostgresSaver

def create_production_app():
    # Load configuration
    db_url = os.getenv("DATABASE_URL")
    openai_api_key = os.getenv("OPENAI_API_KEY")

    # Initialize components
    checkpointer = PostgresSaver.from_conn_string(db_url)

    # Build workflow with production settings
    workflow = StateGraph(ProductionState)
    # ... add nodes and edges

    app = workflow.compile(
        checkpointer=checkpointer,
        # Enable interrupts for human-in-the-loop
        interrupt_before=["human_approval"],
        interrupt_after=["critical_action"]
    )

    return app
```

### 2. Monitoring and Logging
```python
import logging
from datetime import datetime

class LoggingMiddleware:
    def __init__(self, logger_name="langgraph"):
        self.logger = logging.getLogger(logger_name)

    def __call__(self, func):
        def wrapper(state):
            start_time = datetime.now()
            self.logger.info(f"Starting {func.__name__} at {start_time}")

            try:
                result = func(state)
                duration = datetime.now() - start_time
                self.logger.info(
                    f"Completed {func.__name__} in {duration.total_seconds():.2f}s"
                )
                return result
            except Exception as e:
                self.logger.error(f"Error in {func.__name__}: {str(e)}")
                raise

        return wrapper

# Apply to nodes
@LoggingMiddleware()
def production_node(state: AgentState) -> AgentState:
    # Your node logic here
    pass
```

