pub struct Object {
    chunks: std::collections::HashMap<super::ChunkCoord, super::Chunk>
}

impl Object {
    pub fn test(n: usize) -> Self {
        let mut chunks = std::collections::HashMap::new();
        for x in 0..n {
            for y in 0..n {
                for z in 0..n {
                    let coord = super::ChunkCoord::new(x, y, z);
                    chunks.insert(coord, super::Chunk::test1());
                }
            }
        }
        Self {
            chunks
        }
    }
}