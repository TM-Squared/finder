pub const IGNORED_DIRS: &[&str] = &[
    ".git", "node_modules", "target", ".vscode", 
    "__pycache__", ".idea", "build", "dist", ".venv"
];

pub const IGNORED_EXTENSIONS: &[&str] = &[
    "exe", "dll", "bin", "so", "o", "obj",
    "zip", "rar", "7z", "tar", "gz", "bz2", "xz",
    "iso", "img", "jpg", "jpeg", "png", "gif", "bmp", "svg", "ico",
    "mp3", "wav", "flac", "ogg", "mp4", "avi", "mkv", "mov",
    "db", "sqlite", "mdb"
];

pub const PLAIN_TEXT_EXTENSIONS: &[&str] = &[
    "txt", "log", "md", "csv", "json", "xml", "html", "htm", 
    "css", "js", "yml", "yaml", "toml", "ini", "conf", "sql", 
    "sh", "bat", "ps1", "py", "rs", "java", "c", "cpp", "h", "hpp"
];