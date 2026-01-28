## Common Patterns

### 1. Form Handling

```slint
export component LoginForm inherits Window {
    callback login(string, string);

    property <string> username: "";
    property <string> password: "";
    property <bool> loading: false;

    VerticalLayout {
        spacing: 16px;
        padding: 24px;

        Text {
            text: "Login";
            font-size: 24px;
            font-weight: bold;
        }

        TextInput {
            text: username;
            placeholder-text: "Username";
            edited => { username = self.text; }
        }

        TextInput {
            text: password;
            placeholder-text: "Password";
            input-type: password;
            edited => { password = self.text; }
        }

        Button {
            text: loading ? "Logging in..." : "Login";
            enabled: !loading && username != "" && password != "";
            clicked => {
                loading = true;
                root.login(username, password);
            }
        }
    }
}
```

### 2. Navigation Pattern

```slint
export component NavigationContainer inherits Window {
    property <int> current-screen: 0;

    @screens := [
        HomeScreen {},
        SettingsScreen {},
        ProfileScreen {}
    ];

    @screen-titles := ["Home", "Settings", "Profile"];

    Rectangle {
        height: 60px;
        background: #3498db;

        HorizontalLayout {
            padding: 16px;

            for i in 0..3 : int {
                Button {
                    text: @screen-titles[i];
                    background: current-screen == i ? #2980b9 : transparent;
                    clicked => { current-screen = i; }
                }
            }
        }
    }

    Rectangle {
        y: 60px;
        height: parent.height - 60px;

        // Show current screen
        @screens[current-screen];
    }
}
```

