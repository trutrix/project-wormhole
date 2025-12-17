



pub trait BA2Traits {
    fn read_file_from_path(path: &str) -> Option<Vec<u8>>;
    fn write_file_to_path(path: &str, data: &[u8]) -> bool;
    fn read_file_from_hash(hash: u64) -> Option<Vec<u8>>;
    fn write_file_to_hash(hash: u64, data: &[u8]) -> bool;
}