pub fn get_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    let factor = 1024.0;
    let units = ["B", "K", "M", "G", "T", "P"];
    
    for unit in units.iter() {
        if size < factor {
            return format!("{:.2}{}", size, unit);
        }
        size /= factor;
    }
    format!("{:.2}P", size)
}