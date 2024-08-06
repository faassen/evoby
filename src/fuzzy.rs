// a fuzzy match algorithm for 32 bit patterns
//
// patterns that have the same bit pattern are more likely to match
// than patterns that are some bits different, to within a maximum distance.
//
// we can use a subset of the match, from a particular index
// into the stored patterns.

use rand::Rng;

struct FuzzyBitMap<V> {
    data: Vec<(u32, V)>,
    max_distance: u32,
    match_chance: f64,
}

impl<V> FuzzyBitMap<V> {
    pub(crate) fn new(max_distance: u32, match_chance: f64) -> FuzzyBitMap<V> {
        FuzzyBitMap {
            data: Vec::new(),
            max_distance,
            match_chance,
        }
    }

    pub(crate) fn insert(&mut self, pattern: u32, value: V) {
        self.data.push((pattern, value));
    }

    pub(crate) fn matching(&self, pattern: u32, index: usize) -> Vec<&V> {
        let mut matching_patterns = Vec::new();
        for i in index..self.data.len() {
            let (stored_pattern, value) = &self.data[i];
            let distance = hamming_distance(pattern, *stored_pattern);
            if distance <= self.max_distance {
                matching_patterns.push((distance, value));
            }
        }
        // sort by distance, making lower distances sort earlier
        matching_patterns.sort_by(|a, b| a.0.cmp(&b.0));
        matching_patterns
            .into_iter()
            .map(|(_, value)| value)
            .collect()
    }

    pub(crate) fn get(&self, pattern: u32, index: usize, rng: &mut impl Rng) -> Option<&V> {
        // go through the list of matching patterns. prefer the ones earlier in the
        // list to later ones. In other words, there's a slight chance we don't match.
        for value in self.matching(pattern, index) {
            if rng.gen_bool(self.match_chance) {
                return Some(value);
            }
        }
        None
    }
}

fn hamming_distance(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_from_index_0_with_max_distance_0() {
        let mut fuzzy_bitmap = FuzzyBitMap::new(0, 0.5);
        fuzzy_bitmap.insert(0b0000, 0);
        fuzzy_bitmap.insert(0b0001, 1);
        fuzzy_bitmap.insert(0b0010, 2);
        fuzzy_bitmap.insert(0b0011, 3);
        fuzzy_bitmap.insert(0b0100, 4);
        fuzzy_bitmap.insert(0b0101, 5);
        fuzzy_bitmap.insert(0b0110, 6);
        fuzzy_bitmap.insert(0b0111, 7);
        // now look for matches
        let matches = fuzzy_bitmap.matching(0b0000, 0);
        assert_eq!(matches, vec![&0b0000]);
    }

    #[test]
    fn test_matching_from_index_0_with_max_distance_1() {
        let mut fuzzy_bitmap = FuzzyBitMap::new(1, 0.5);
        fuzzy_bitmap.insert(0b0000, 0);
        fuzzy_bitmap.insert(0b0001, 1);
        fuzzy_bitmap.insert(0b0010, 2);
        fuzzy_bitmap.insert(0b0011, 3);
        fuzzy_bitmap.insert(0b0100, 4);
        fuzzy_bitmap.insert(0b0101, 5);
        fuzzy_bitmap.insert(0b0110, 6);
        fuzzy_bitmap.insert(0b0111, 7);
        // now look for matches
        let matches = fuzzy_bitmap.matching(0b0000, 0);
        assert_eq!(matches, vec![&0b0000, &0b0001, &0b0010, &0b0100]);
    }

    #[test]
    fn test_matching_from_index_0_with_max_distance_2() {
        let mut fuzzy_bitmap = FuzzyBitMap::new(2, 0.5);
        fuzzy_bitmap.insert(0b0000, 0);
        fuzzy_bitmap.insert(0b0001, 1);
        fuzzy_bitmap.insert(0b0010, 2);
        fuzzy_bitmap.insert(0b0011, 3);
        fuzzy_bitmap.insert(0b0100, 4);
        fuzzy_bitmap.insert(0b0101, 5);
        fuzzy_bitmap.insert(0b0110, 6);
        fuzzy_bitmap.insert(0b0111, 7);
        // now look for matches
        let matches = fuzzy_bitmap.matching(0b0000, 0);
        assert_eq!(
            matches,
            vec![&0b0000, &0b0001, &0b0010, &0b0100, &0b0011, &0b0101, &0b0110]
        );
    }
}
