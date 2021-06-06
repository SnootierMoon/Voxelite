#[derive(Default)]
pub struct Chunk {
    blocks: Box<[[[super::Block; Self::SIZE]; Self::SIZE]; Self::SIZE]>
}

impl Chunk {
    pub const SIZE: usize = 32;
    pub const SIZE2: usize = Self::SIZE * Self::SIZE;
    pub const SIZE3: usize = Self::SIZE * Self::SIZE * Self::SIZE;

    pub fn test1() -> Self {
        let mut chunk = Self::default();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    if (ultraviolet::Vec3::new(x as f32, y as f32, z as f32) - ultraviolet::Vec3::new(16., 16., 16.)).mag_sq() < 16.*16. {
                        chunk.blocks[x][y][z] = 1
                    }
                }
            }
        }
        chunk
    }

    pub fn test2() -> Self {
        let mut chunk = Self::default();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    chunk.blocks[x][y][z] = 1
                }
            }
        }
        chunk
    }

    fn face(x: usize, y: usize, z: usize, f: usize) -> u32 {
        (x << 0 | y << 5 | z << 10 | f << 15) as u32
    }

    pub fn faces(&self) -> Vec<u32> {
        let mut faces = Vec::new();
        for x in 0..Self::SIZE {
            for y in 0..Self::SIZE {
                for z in 0..Self::SIZE {
                    if self.get2(x, y, z) {
                        if !self.get2(x+1, y, z) { faces.push(Self::face(x, y, z, 0)) }
                        if !self.get2(x-1, y, z) { faces.push(Self::face(x, y, z, 1)) }
                        if !self.get2(x, y+1, z) { faces.push(Self::face(x, y, z, 2)) }
                        if !self.get2(x, y-1, z) { faces.push(Self::face(x, y, z, 3)) }
                        if !self.get2(x, y, z+1) { faces.push(Self::face(x, y, z, 4)) }
                        if !self.get2(x, y, z-1) { faces.push(Self::face(x, y, z, 5)) }
                    }
                }
            }
        }

        faces
    }

    pub fn get2(&self, x: usize, y: usize, z: usize) -> bool {
        self.get(x, y, z).unwrap_or(false)
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<bool> {
        Some(*self.blocks.get(x)?.get(y)?.get(z)? != 0)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

impl Coord {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}