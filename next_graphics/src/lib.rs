use noa::{log::log::*,noa_ui_log};
use slint::{SharedString, Weak};


slint::include_modules!();

pub fn show_main_window(logger:&'static dyn UILogger, main_title:String) ->Result<(), slint::PlatformError>
{
    let result = MainWindow::new();
    let ui = result.and_then(|window|
    {
        noa_ui_log!(logger, LogLevel::INFO("UI is initialized"), stringify!(show_main_window()));
        Ok(window)
    })?;
    
    let _ui_handle = ui.as_weak();

    let _title = SharedString::from(main_title);

    init_ui_callbacks(_ui_handle, logger);
    

    ui.invoke_window_init(_title);

    ui.run()

}


fn init_ui_callbacks(ui_handler: Weak<MainWindow>, logger:&'static dyn UILogger)
{

    let ui = ui_handler.unwrap();

    let main = ui_handler.unwrap();

    ui.on_window_init(move|title|
    {
        let ui = ui_handler.unwrap();
        ui.set_app_text(title);
        noa_ui_log!(logger, LogLevel::INFO("Slint UI is initialized"), stringify!(show_main_window()));
    });

    ui.on_button1_click(move||
    {
        noa_ui_log!(logger, LogLevel::DEBUG("button cliked"), stringify!(ui.on_button1_click()));
    });

    ui.on_window_close(move ||
    {
        main.hide().unwrap();
    });

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

