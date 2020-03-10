#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualOffset(u64);

/// Chunk `[start-end)`, where `start` and `end` are [virtual offsets](struct.VirtualOffset.html).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chunk {
    sample_id: u32,
    file_id: u32, // When we split files of inverted data structure
    // format_type: u32, // Enum
    start: VirtualOffset,
    end: VirtualOffset, // Or, "length"?
}

/// Single bin that stores chunks of 
#[derive(Debug, Eq)]
pub struct Bin {
    bin_id: u32,
    chunks: Vec<Chunk>, // chunks for each sample, each format type
}

/// Index for a single reference sequence. Contains [bins](struct.Bin.html).
#[derive(Clone)]
pub struct Reference {
    bins: HashMap<u32, Bin>, // bin_id is not continuous.
    // linear_index: LinearIndex,
}

#[derive(Clone)]
pub struct Index {
    samples: Vec<String>, // sample names, we expect sample id is continuous.
    references: Vec<Reference>, // the length is inherited from n_ref
}
