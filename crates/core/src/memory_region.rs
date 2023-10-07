use std::error::Error;

#[derive(Debug)]
pub struct MemoryRegion {
    pub start: usize,
    pub size: usize,
}

impl MemoryRegion {
    pub fn new(start: usize, size: usize, skip_check: bool) -> MemoryRegion {
        if !skip_check {
            // TODO: Check if the region is valid
        }
        MemoryRegion { start, size }
    }
    pub fn to_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.start as *const u8, self.size) }
    }
}

impl MemoryRegion {
    pub fn from_executable() -> Result<Vec<MemoryRegion>, Box<dyn Error>> {
        let current_pid = std::process::id();
        let maps = proc_maps::get_process_maps(current_pid as proc_maps::Pid)?;
        let primary_module_name = maps.first().unwrap().filename().unwrap().as_os_str();
        let ret = maps
            .iter()
            .filter(|m| {
                let filename_osstr = match m.filename() {
                    Some(filename) => filename.as_os_str(),
                    None => std::ffi::OsStr::new(""),
                };
                filename_osstr == primary_module_name && m.is_exec() && m.is_read()
            })
            .map(|m| MemoryRegion::new(m.start(), m.size(), false))
            .collect();
        Ok(ret)
    }
}
