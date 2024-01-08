pub mod log
{
    pub mod log;
}

pub use log::log::ui_logger as ui_logger;
pub use log::log::NoaLoggerConfig as NoaLoggerConfig;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
