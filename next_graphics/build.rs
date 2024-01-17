fn main()
{
    build_pxiel_ui();
    
}

fn build_pxiel_ui()
{
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());

    slint_build::compile_with_config("ui/mainPixel.slint", config).unwrap();
    //slint_build::compile("ui/mainPixel.slint").unwrap();
}

fn build_modern_ui()
{
    // TODO: Add modern UI in future
}