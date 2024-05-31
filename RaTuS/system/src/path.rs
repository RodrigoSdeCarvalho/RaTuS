use std::path;
use std::env;
use std::ffi::OsStr;
use std::sync::{Mutex, MutexGuard, Once};
use std::fmt::Display;

pub type SysPath = path::PathBuf;

static ROOT_NAME: &str = "RaTuS";
static mut PATH: Option<Mutex<Path>> = None;
static SINGLETON: Once = Once::new();

pub struct Path {
    root: SysPath,
}

#[macro_export]
macro_rules! join_root {
    ($($arg:expr),*) => {
        Path::join_root(vec![$($arg),*])
    };
}
pub(crate) use join_root;

impl Path {
    pub fn get_data<T> (file_name: T) -> SysPath
        where T: Display
    {
        let data_path: SysPath = join_root!("assets", "data");
        let data_file_path = Path::join(&data_path, file_name.to_string());

        data_file_path
    }

    pub fn get_model<T>(file_name: T) -> SysPath
        where T: Display
    {
        let model_path = Path::join(&Path::get_models(), file_name.to_string());

        model_path
    }

    pub fn get_models() -> SysPath {
        let models_path: SysPath = join_root!("assets", "models");

        models_path
    }

    pub fn get_assets() -> SysPath {
        let assets_path: SysPath = join_root!("assets");

        assets_path
    }

    pub fn join_root(file_folder_names: Vec<&str>) -> SysPath {
        let path: MutexGuard<Path> = Path::get().lock().unwrap();
        let mut joined_path: SysPath = path.root.clone();

        for file_folder_name in file_folder_names {
            joined_path.push(file_folder_name);
        }

        joined_path
    }

    fn join(path: &SysPath, file_folder_name:String) -> SysPath {
        let mut joined_path: SysPath = path.clone();
        joined_path.push(file_folder_name);

        joined_path
    }

    fn get<'a>() -> &'a Mutex<Path> { // Will be unlocked for as long as the MutexGuard is in the caller's scope
        SINGLETON.call_once(|| {
            let root: SysPath = Path::find_root();
            unsafe {
                PATH = Some(Mutex::new(Path { root }));
            }
        });

        unsafe {
            PATH.as_ref()
                .unwrap()

        }
    }

    fn find_root() -> SysPath {
        let mut root: SysPath = env::current_exe()
            .unwrap();

        let mut tries: u8 = 0;
        while root.file_name() != Some(OsStr::new(ROOT_NAME)) {
            tries += 1;
            if tries > 10 {
                panic!("Could not find root directory");
            }
            root = root.parent()
                .unwrap()
                .to_path_buf();
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_root() {
        let executable_path: SysPath = env::current_exe().unwrap();

        let mut root: SysPath = executable_path.clone();
        for _ in 0..4 { // Goes back from target/debug/deps to the root directory
            root = root.parent().unwrap().to_path_buf();
        }

        let found_root: SysPath = Path::find_root();

        assert_eq!(root, found_root);
    }

    #[test]
    fn test_joins() {
        let root: SysPath = Path::find_root();
        let joined_path: SysPath = join_root!("assets");

        assert_eq!(joined_path, Path::join(&root, "assets".to_string()));
    }

    #[test]
    fn test_get_folders() {
        let assets_path = Path::get_assets();
        println!("{:?}", assets_path);

        let models_path = Path::get_models();
        println!("{:?}", models_path);

        let llama2_path = Path::get_model("Llama2");
        println!("{:?}", llama2_path);

        let data_path = Path::get_data("data.csv");
        println!("{:?}", data_path);
    }
}
