/// Which Yaz1 compressor variant is used to (de)compress a ghost's input data.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    /// Vanilla Mario Kart Wii's standard Yaz1 compressor.
    Vanilla,
    /// CTGP-R's Yaz1 variant: all-literal encoding, no back-reference search.
    CTGP,
    /// MKW-SP's Yaz1 variant: greedy back-reference matching, no lookahead.
    SP,
}
