slint::include_modules!();

pub fn show_main_window() ->Result<(), slint::PlatformError>
{
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    let ui_handle = ui.as_weak();
    ui.on_request_increase_value(move || {
        let ui = ui_handle.unwrap();
        ui.set_counter(ui.get_counter() + 1);
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
