mod utils;

use utils::memory::SystemMemory;

fn main() {
    let sysmem = SystemMemory::new();
    println!("{sysmem:?}");
}
