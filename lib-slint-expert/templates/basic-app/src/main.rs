slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;

    // Set up event handlers
    let window_weak = main_window.as_weak();
    main_window.on_counter_button_clicked(move || {
        let window = window_weak.unwrap();
        let current_count = window.get_counter_value();
        window.set_counter_value(current_count + 1);
    });

    let window_weak = main_window.as_weak();
    main_window.on_reset_button_clicked(move || {
        let window = window_weak.unwrap();
        window.set_counter_value(0);
    });

    main_window.run()
}