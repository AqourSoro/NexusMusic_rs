
use noa::{log::log::*,noa_ui_log};


slint::include_modules!();

pub fn show_main_window(logger:&'static dyn UILogger) ->Result<(), slint::PlatformError>
{
    let result = MainWindow::new();
    let ui = result.and_then(|window|
    {
        noa_ui_log!(logger, LogLevel::INFO("UI is initialized"), stringify!(show_main_window()));
        Ok(window)
    })?;
    
    let _ui_handle = ui.as_weak();

    ui.on_button_click(move||
    {
        noa_ui_log!(logger, LogLevel::DEBUG("button cliked"), stringify!(ui.on_button_click()));
    });

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

