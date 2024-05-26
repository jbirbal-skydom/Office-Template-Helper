fn main() {
    #[cfg(windows)]
    {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("assets/icons/logo.ico");
    res.compile().unwrap();
    //windres::Build::new().compile("app_icon.rc").unwrap();
    //    println!("cargo:rustc-link-arg=app_icon.res");
    }
    
    slint_build::compile("ui/appwindow.slint").unwrap();


    // let build_date = String::from_utf8(output.stdout)
    //     .expect("Output was not UTF-8")
    //     .trim()
    //     .to_string();
    // println!("cargo:rustc-env=BUILD_DATE={}", build_date);
    // writeln!(file, "Running build date: {}", build_date).expect("Failed to write to log file"); // Embedding the build date


}