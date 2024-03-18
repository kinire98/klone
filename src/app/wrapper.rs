pub struct Wrapper;
impl Drop for Wrapper {
    fn drop(&mut self) {
        crate::output::clear_line();
        println!("Backup finished");
    }
}
