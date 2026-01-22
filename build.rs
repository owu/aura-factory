fn main() {
    slint_build::compile("ui/app.slint").unwrap();
    
    // 仅在 Windows 平台嵌入资源文件
    #[cfg(windows)]
    {
        // 转换 PNG 到 ICO
        use image::ImageReader;
        use std::path::Path;
        
        let png_path = Path::new("ui/statics/logo.png");
        let ico_path = Path::new("ui/statics/logo.ico");
        
        // 读取 PNG 并调整大小后保存为 ICO
        let img = ImageReader::open(png_path)
            .expect("Failed to open PNG file")
            .decode()
            .expect("Failed to decode PNG");
        
        // 调整图片大小到 256x256（ICO 最大尺寸）
        let resized_img = img.resize_to_fill(256, 256, image::imageops::FilterType::Lanczos3);
        
        resized_img.save(ico_path)
            .expect("Failed to save ICO file");
        
        // 读取 src/consts.rs 文件，获取 APP_VERSION
        let consts_content = std::fs::read_to_string("src/consts.rs").expect("Failed to read src/consts.rs");
        let version_regex = regex::Regex::new(r#"APP_VERSION: &str = "([^"]+)""#).expect("Failed to create regex");
        let version = version_regex.captures(&consts_content)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str())
            .unwrap_or("0.0.2");
        
        // 将版本号拆分为主要、次要、补丁版本号
        let mut version_parts = version.split('.').map(|p| p.parse::<u16>().unwrap_or(0)).collect::<Vec<_>>();
        while version_parts.len() < 3 {
            version_parts.push(0);
        }
        let (major, minor, patch) = (version_parts[0], version_parts[1], version_parts[2]);
        
        // 更新 icon.rc 文件，添加 VERSIONINFO 资源
        std::fs::write(
            "src/icon.rc",
            format!(r#"#include <windows.h>

IDI_ICON1 ICON "../ui/statics/logo.ico"

VS_VERSION_INFO VERSIONINFO
 FILEVERSION {major},{minor},{patch},0
 PRODUCTVERSION {major},{minor},{patch},0
 FILEFLAGSMASK 0x3fL
#ifdef _DEBUG
 FILEFLAGS 0x1L
#else
 FILEFLAGS 0x0L
#endif
 FILEOS 0x40004L
 FILETYPE 0x1L
 FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904b0"
        BEGIN
            VALUE "CompanyName", "Aura Factory"
            VALUE "FileDescription", "Aura Factory - Video processing application"
            VALUE "FileVersion", "{major}.{minor}.{patch}.0"
            VALUE "InternalName", "aura-factory"
            VALUE "LegalCopyright", "2025 Aura Factory. All rights reserved."
            VALUE "OriginalFilename", "AuraFactory.exe"
            VALUE "ProductName", "Aura Factory"
            VALUE "ProductVersion", "{major}.{minor}.{patch}.0"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1200
    END
END
"#)
        ).expect("Failed to write icon.rc");
        
        // 编译资源文件
        embed_resource::compile("src/icon.rc", std::iter::empty::<&std::ffi::OsStr>());
    }
}
