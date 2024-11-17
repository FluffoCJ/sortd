use std::fs;
use dirs::home_dir;


fn main() {
    let home = home_dir().expect("Failed to get home directory");
    let downloads_path = home.join("Downloads");

    let zipped_path = downloads_path.join("Zipped");
    let audio_path = downloads_path.join("Audio");
    let video_path = downloads_path.join("Videos");
    let images_path = downloads_path.join("Images");
    let folders_path = downloads_path.join("Folders");
    let etc_path = downloads_path.join("Etc");
    
    let paths = [
        &zipped_path,
        &audio_path,
        &video_path,
        &images_path,
        &folders_path,
        &etc_path,
    ];
    
    for path in paths {
        if !path.exists() {
            fs::create_dir_all(path).expect(&format!("Failed to create {:?}", path));
        }
    }

        let zipped_extensions = [".zip", ".tar", ".7z"];
        let audio_extensions = [".mp3", ".aac", ".ogg", ".m4a", ".wav", ".mid", ".midi", ".ogx"];
        let video_extensions = [".mp4", ".avi", ".mov", ".mkv", ".flv", ".mpg", ".hevc", ".ogv", ".webm"];
        let image_extensions = [".png", ".jpg", ".jpeg", ".webp", ".ico", ".gif", ".tiff", ".tif", ".svg", ".hdr"];


    for file in fs::read_dir(downloads_path).unwrap() {
        let file = file.unwrap();
        let file_path = file.path();

        let file_name = file.file_name();
        for element in zipped_extensions {
            if file_path.to_string_lossy().contains(element) {
                fs::rename(&file_path, zipped_path.join(file_name.clone())).expect("Failed to move zip file");
            }
        }
        for element in audio_extensions {
            if file_path.to_string_lossy().contains(element) {
                fs::rename(&file_path, audio_path.join(file_name.clone())).expect("Failed to move audio file");
            }
        }
        for element in video_extensions {
            if file_path.to_string_lossy().contains(element) {
                fs::rename(&file_path, video_path.join(file_name.clone())).expect("Failed to move video file");
            }
        }
        for element in image_extensions {
            if file_path.to_string_lossy().contains(element) {
                fs::rename(&file_path, images_path.join(file_name.clone())).expect("Failed to move image file");
            }
        }
        if !file_path.is_file() &&
        !file_path.to_string_lossy().contains(".") &&
        !file_path.to_string_lossy().contains("Images") &&
        !file_path.to_string_lossy().contains("Videos") &&
        !file_path.to_string_lossy().contains("Etc") &&
        !file_path.to_string_lossy().contains("Audio") &&
        !file_path.to_string_lossy().contains("Folders") &&
        !file_path.to_string_lossy().contains("Zipped") {
            fs::rename(&file_path, folders_path.join(file_name.clone())).expect("Failed to move folder");
        }
        else if file_path.is_file() {
            fs::rename(&file_path, etc_path.join(file_name.clone())).expect("Failed to move file");
        }
    }
}
