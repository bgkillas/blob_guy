use crate::CHUNK_SIZE;
use crate::noita::pixel::NoitaPixelRun;
pub struct Chunk {
    pixels: [u16; CHUNK_SIZE * CHUNK_SIZE],
}
impl From<Vec<NoitaPixelRun>> for Chunk {
    fn from(value: Vec<NoitaPixelRun>) -> Self {
        let mut pixels = [0; CHUNK_SIZE * CHUNK_SIZE];
        let mut i = 0;
        for v in value {
            for _ in 0..v.length {
                pixels[i] = v.material;
                i += 1;
            }
        }
        Chunk { pixels }
    }
}
#[derive(Copy, Clone)]
pub enum CellType {
    Solid,
    Liquid(LiquidType),
    Gas,
    Fire,
    Invalid,
}
impl CellType {
    pub fn new(s: &str, stat: bool, sand: bool) -> Self {
        match s {
            "solid" => Self::Solid,
            "liquid" if stat => Self::Liquid(LiquidType::Static),
            "liquid" if sand => Self::Liquid(LiquidType::Sand),
            "liquid" => Self::Liquid(LiquidType::Liquid),
            "gas" => Self::Gas,
            "fire" => Self::Fire,
            _ => Self::Invalid,
        }
    }
    fn can_remove(&self, hole: bool, liquid: bool) -> bool {
        match self {
            Self::Liquid(LiquidType::Sand) | Self::Liquid(LiquidType::Static) | Self::Solid
                if hole =>
            {
                true
            }
            Self::Liquid(LiquidType::Liquid) if liquid => true,
            _ => false,
        }
    }
}
#[derive(Copy, Clone)]
pub enum LiquidType {
    Static,
    Liquid,
    Sand,
}
