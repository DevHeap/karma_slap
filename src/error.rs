error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Json(::json::Error);
        AppDirs(::app_dirs::AppDirsError);
    }
}