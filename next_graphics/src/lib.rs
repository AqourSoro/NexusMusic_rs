
use noa::log::log::*;


slint::include_modules!();

pub fn show_main_window(logger:&NexusLogger) ->Result<(), slint::PlatformError>
{
    let ui = MainWindow::new()?;

    let ui_handle_weak = ui.as_weak();
    UIlogger::log(logger, LogLevel::INFO("UI is running now"), stringify!(show_main_window()));

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

