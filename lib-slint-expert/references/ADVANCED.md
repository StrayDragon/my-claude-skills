## Advanced Topics

### 1. Custom Components Library

**Component Library Structure**
```slint
// components/button.slint
export component PrimaryButton inherits Rectangle {
    property <string> text;
    property <bool> enabled: true;
    callback clicked;

    background: enabled ? #3498db : #bdc3c7;
    border-radius: 6px;

    Text {
        text: root.text;
        color: white;
        horizontal-alignment: center;
        vertical-alignment: center;
    }

    TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}

// components/card.slint
export component Card inherits Rectangle {
    property <string> title;
    property <string> content;

    background: white;
    border-width: 1px;
    border-color: #ecf0f1;
    border-radius: 8px;
    elevation: 2dp;

    VerticalLayout {
        padding: 16px;
        spacing: 8px;

        Text {
            text: title;
            font-weight: bold;
            font-size: 18px;
        }

        Text {
            text: content;
            color: #7f8c8d;
            wrap: word-wrap;
        }
    }
}
```

### 2. Performance Optimization

**Efficient Rendering**
```slint
export component OptimizedList inherits Window {
    property <[ListItem]> items;

    ListView {
        for item in items : ListItem {
            height: 60px;

            Rectangle {
                background: even ? #f8f9fa : white;

                Text {
                    x: 16px;
                    text: item.title;
                    vertical-alignment: center;
                }
            }
        }
    }
}

// In Rust - implement efficient data models
use slint::{ModelRc, VecModel};

struct ListItem {
    title: String,
    // other fields
}

impl From<ListItem> for slint::ModelRc<slint::SharedString> {
    fn from(items: Vec<ListItem>) -> Self {
        let model = VecModel::from(
            items.into_iter()
                .map(|item| slint::SharedString::from(item.title))
                .collect()
        );
        ModelRc::new(model)
    }
}
```

**Memory Management**
```rust
// Use weak references to avoid circular references
let window_weak = app.as_weak();

// Clear models when no longer needed
app.on_cleanup(move || {
    if let Some(window) = window_weak.upgrade() {
        window.set_data_model(ModelRc::new(VecModel::default()));
    }
});

// Use resource pooling for frequently created components
struct ComponentPool {
    buttons: Vec<Rc<slint::ComponentHandle<ButtonComponent>>>,
}

impl ComponentPool {
    fn get_button(&mut self) -> Rc<slint::ComponentHandle<ButtonComponent>> {
        self.buttons.pop().unwrap_or_else(|| {
            Rc::new(ButtonComponent::new().unwrap())
        })
    }

    fn return_button(&mut self, button: Rc<slint::ComponentHandle<ButtonComponent>>) {
        self.buttons.push(button);
    }
}
```

### 3. Cross-Platform Development

**Platform-Specific Configuration**
```rust
// build.rs
fn main() {
    let mut config = slint_build::CompilerConfiguration::new();

    // Configure platform-specific features
    #[cfg(target_os = "windows")]
    {
        config = config.with_style("fluent");
    }

    #[cfg(target_os = "macos")]
    {
        config = config.with_style("native");
    }

    #[cfg(target_arch = "wasm32")]
    {
        config = config.with_style("material");
    }

    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
```

**WebAssembly Integration**
```rust
// main.rs
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    let main_window = MainWindow::new().unwrap();

    // WASM-specific setup
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .style()
            .set_property("margin", "0")
            .unwrap();
    }

    main_window.run().unwrap();
}
```

**Embedded/MCU Support**
```rust
#![no_std]
#![cfg_attr(not(feature = "simulator"), no_main)]

slint::include_modules!();

#[cfg_attr(not(feature = "simulator"), no_main)]
fn main() -> ! {
    mcu_board_support::init();

    let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
        slint::platform::software_renderer::RepaintBufferType::ReusedBuffer
    );

    let ui = MainWindow::new().unwrap();

    // Embedded-specific event loop
    loop {
        ui.window().draw_if_needed(|renderer| {
            renderer.render_by_line(&mut display_wrapper);
        });

        // Handle other embedded tasks
        mcu_board_support::delay_ms(16); // ~60 FPS
    }
}
```

## Best Practices

### 1. Code Organization

**Project Structure**
```
my-app/
├── Cargo.toml
├── build.rs
├── src/
│   ├── main.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── data.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── button.rs
│   │   └── card.rs
│   └── ui/
│       ├── main.slint
│       ├── components/
│       │   ├── button.slint
│       │   └── card.slint
│       └── styles/
│           └── theme.slint
└── assets/
    ├── icons/
    └── images/
```

**Component Design Principles**
- Keep components focused and reusable
- Use properties for configuration
- Prefer composition over inheritance
- Implement proper error handling
- Use TypeScript-like naming conventions

### 2. Performance Guidelines

**Rendering Optimization**
- Use `clip: true` for complex shapes
- Avoid unnecessary property animations
- Implement efficient data models
- Use `ListView` for large datasets
- Minimize layout recalculations

**Memory Management**
- Use weak references for callbacks
- Clear models when components are destroyed
- Implement resource pooling for reusable components
- Avoid circular references between Rust and Slint

### 3. Testing Strategies

**Unit Testing**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use slint_testing::*;

    #[test]
    fn test_button_click() {
        let app = TestApp::new().unwrap();

        // Simulate button click
        app.get_test_button().clicked().emit();

        // Verify state changes
        assert_eq!(app.get_click_count(), 1);
    }

    #[test]
    fn test_data_binding() {
        let app = TestApp::new().unwrap();
        app.set_input_text("Hello");

        assert_eq!(app.get_display_text(), "Hello");
    }
}
```

