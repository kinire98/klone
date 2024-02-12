pub fn should_be_backed<T>(file_to_backup: T, already_backed_file: T) -> bool
where
    T: File,
{
    if already_backed_file.just_created() {
        return true;
    }
    #[cfg(windows)]
    if file_to_backup.is_folder() {
        return true;
    }
    file_to_backup.get_time() > already_backed_file.get_time()
}
pub trait File {
    fn get_time(&self) -> i128;
    fn just_created(&self) -> bool;
    #[cfg(windows)]
    fn is_folder(&self) -> bool;
}
#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;
