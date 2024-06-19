
use {
    std::{
        env,
        io,
    },
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    let config: slint_build::CompilerConfiguration =
    slint_build::CompilerConfiguration::new()
        .with_style("cosmic-dark".into());
    slint_build::compile_with_config("ui/template.slint", config).unwrap();
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/pdf.ico")
            .set("FileVersion", "1.0.0")
            .set("FileDescription", "PDF Searcher")
            .set("ProductName", "PDF Searcher")
            .set("ProductVersion", "1.0.0")
            .set("LegalCopyright", "Copyright © 2024")
            .compile()?;
    }
    Ok(())
}