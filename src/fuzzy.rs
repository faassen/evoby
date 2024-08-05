// a fuzzy match algorithm for 32 bit patterns
//
// patterns that have the same bit pattern are more likely to match
// than patterns that are some bits different, to within a maximum distance.
//
// we can use a subset of the match, from a particular index
// into the stored patterns.

struct FuzzyBitMap<V> {
    data: Vec<(u32, V)>,
    max_distance: u32,
}

impl<V> FuzzyBitMap<V> {
    pub(crate) fn new(max_distance: u32) -> FuzzyBitMap<V> {
        FuzzyBitMap {
            data: Vec::new(),
            max_distance,
        }
    }

    pub(crate) fn insert(&mut self, pattern: u32, value: V) {
        self.data.push((pattern, value));
    }

    // this needs to be passed a random number generator
    pub(crate) fn matches(&self, pattern: u32, index: usize) -> Option<&V> {
        let mut matching_patterns = Vec::new();
        for i in index..self.data.len() {
            let (stored_pattern, value) = &self.data[i];
            let distance = hamming_distance(pattern, *stored_pattern);
            if distance <= self.max_distance {
                // instead of doing this, match on chance based on hamming
                // dinstance, where hamming distance of 0 has very low chance of
                // mismatch, hamming distance of 1 is higher, etc.
                matching_patterns.push((distance, value));
            }
        }
        todo!()
    }
}

fn hamming_distance(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}
