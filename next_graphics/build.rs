use slint_build::CompilerConfiguration;

fn main()
{

    let pixel_config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());

    let pixel_path = "ui/pixel_windows/mainPixel.slint";

    let pixel_style = (pixel_path, pixel_config);

    build_ui(pixel_style);
    
}


fn build_ui((path, config): (&str, CompilerConfiguration))
{
    slint_build::compile_with_config(path, config).unwrap();
}