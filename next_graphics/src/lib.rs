use noa::ui_logger;


slint::include_modules!();

pub fn show_main_window() ->Result<(), slint::PlatformError>
{
    let ui = MainWindow::new()?;

    let ui_handle_weak = ui.as_weak();

    ui_logger("UI is running in show_main_window!".to_string());

    ui.run()

}



pub fn add_in_graphics(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_in_graphics(2, 2);
        assert_eq!(result, 4);
    }
}
