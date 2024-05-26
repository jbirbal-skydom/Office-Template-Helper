fn main() {
    #[cfg(windows)]
    {
        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon("assets/icons/logo.ico");
        res.compile().unwrap();
    }

    slint_build::compile("ui/appwindow.slint").unwrap();
}
