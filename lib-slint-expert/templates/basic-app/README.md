# Slint Basic Application Template

A simple, well-structured starting point for Slint GUI applications. This template demonstrates basic concepts including components, properties, callbacks, and event handling.

## Features

- ✅ Clean project structure
- ✅ Basic component composition
- ✅ Property binding and state management
- ✅ Event handling and callbacks
- ✅ Responsive layout with VerticalLayout and HorizontalLayout
- ✅ Modern styling and theming
- ✅ Type-safe Rust integration

## Quick Start

1. **Copy this template** to your project directory
2. **Install dependencies**:
   ```bash
   cargo build
   ```
3. **Run the application**:
   ```bash
   cargo run
   ```

## Project Structure

```
slint-basic-app/
├── Cargo.toml              # Project configuration
├── build.rs                # Build script for Slint compilation
├── README.md               # This file
└── src/
    ├── main.rs             # Main application logic
    └── ui/
        └── main.slint      # UI component definition
```

## How It Works

### UI Definition (`src/ui/main.slint`)

The main window is defined using Slint's declarative language:

- **Component Structure**: `MainWindow` inherits from `Window`
- **Properties**: `counter-value` tracks the current count
- **Callbacks**: Define custom events (`counter-button-clicked`, `reset-button-clicked`)
- **Layout**: Uses `VerticalLayout` and `HorizontalLayout` for responsive design
- **Styling**: Modern colors, borders, and typography

### Rust Integration (`src/main.rs`)

The Rust code handles business logic:

- **Component Loading**: `slint::include_modules!()` includes the UI
- **Event Handlers**: Set up callback handlers for user interactions
- **State Management**: Update UI properties from Rust code
- **Application Loop**: Start the event loop with `run()`

### Build Process (`build.rs`)

The build script compiles Slint files:

- **Compilation**: `slint_build::compile()` processes `.slint` files
- **Integration**: Generated Rust code is linked during compilation

## Customization

### Adding New Components

1. **Create new .slint file** in `src/ui/`
2. **Import in main.slint**:
   ```slint
   import { MyComponent } from "my-component.slint";
   ```
3. **Use in layout**:
   ```slint
   MyComponent {
       // properties
   }
   ```

### Adding New Properties

```slint
// In component definition
property <string> user-name: "Guest";
property <color> theme-color: #3498db;
property <bool> enabled: true;
```

### Adding New Callbacks

```slint
// Define callback
callback my-event(string, int);

// Trigger callback
my-event("hello", 42);
```

### Handling Events in Rust

```rust
// Set up handler
let window_weak = main_window.as_weak();
main_window.on_my_event(move |text, number| {
    let window = window_weak.unwrap();
    // Handle event
    println!("Received: {} {}", text, number);
});
```

## Learning Path

1. **Start Here**: Understand the basic structure
2. **Modify UI**: Change colors, text, and layout
3. **Add Logic**: Implement new event handlers
4. **Extend**: Add new components and features
5. **Explore**: Check out other templates for advanced patterns

## Next Steps

- 🎨 Try the **Component Library Template** for reusable components
- 🌐 Explore the **Cross-Platform Template** for multi-target deployment
- 📚 Read the [Slint Documentation](https://slint.dev/)
- 🧪 Experiment with the examples in the main skill

## Troubleshooting

**Build Issues**:
- Ensure Rust and Cargo are installed
- Check that all dependencies are up to date: `cargo update`

**Runtime Issues**:
- Verify Slint backend is available on your platform
- Check console output for error messages

**UI Not Showing**:
- Ensure window dimensions are set
- Check that components have proper sizing constraints

---

*Ready to build your first Slint application? Start customizing this template now!*