use crate::resources::Resource;

pub struct Pacman { }

impl Resource for Pacman {
    fn new() -> Self {
        Self { }
    }
    fn get_resource(&self) -> Vec<[u32; 17]> {
        vec![
            [15, 0, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0],
            [15, 20, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0],
            [15, 66, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0],
            [15, 42, 20, 0, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0, 0],
            [15, 73, 85, 34, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0],
            [15, 65, 73, 73, 42, 20, 0, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0],
            [15, 69, 65, 73, 85, 34, 0, 0, 8, 0, 0, 0, 0, 8, 0, 0, 0],
            [15, 34, 69, 65, 73, 73, 42, 20, 0, 0, 0, 0, 8, 0, 0, 0, 0],
            [15, 28, 34, 69, 65, 73, 85, 34, 0, 0, 0, 8, 0, 0, 0, 0, 8],
            [15, 0, 28, 34, 69, 65, 73, 73, 42, 20, 8, 0, 0, 0, 0, 8, 0],
            [15, 0, 0, 28, 34, 69, 65, 73, 85, 34, 0, 0, 0, 0, 8, 0, 0],
            [15, 0, 0, 0, 28, 34, 69, 65, 73, 73, 42, 20, 0, 8, 0, 0, 0],
            [15, 0, 0, 0, 0, 28, 34, 69, 65, 73, 85, 34, 8, 0, 0, 0, 0],
            [15, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73, 73, 42, 20, 0, 0, 8],
            [15, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73, 85, 34, 0, 8, 0],
            [15, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73, 73, 42, 20, 0],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73, 85, 34, 0],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73, 73, 42],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73, 85],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65, 73],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69, 65],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34, 69],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 34],
            [15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28]
        ]
    }
}
