use debruijn::kmer::Kmer16;
use smallvec::SmallVec;
use serde::{Serialize, Deserialize};

pub type Anchors = Vec<(u32, u32)>;
pub type Color = u128;

//Use the SmallVec impelementation to save lots of memory 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmerNode{
    pub kmer: Kmer16,
    pub order: u32,
    pub color: Color,
    pub child_nodes: SmallVec<[u32;1]>,
    pub child_edge_distance: SmallVec<[(u16,(Color,u8));1]>,
//    pub child_nodes: Vec<u32>,
    pub id: u32,
    pub canonical: bool,
    pub actual_ref_positions: SmallVec<[usize;0]>,
}
