#[cfg(windows)]
fn main() {
    let mut resource = winres::WindowsResource::new();
    resource.set_icon("resources/dashboard.ico");
    resource
        .set("ProductName", "ExtraVR")
        .set("FileDescription", "ExtraVR Dashboard")
        .set("CompanyName", "ExtraVR")
        .set("OriginalFilename", "ExtraVR Dashboard.exe")
        .set("LegalCopyright", "Based on ALVR — MIT License");
    resource.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
