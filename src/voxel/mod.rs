mod chunk;
mod object;

pub use chunk::{Chunk, Coord as ChunkCoord};
pub use object::Object;

type Block = u16;
