use std::ops::Range;

pub struct RangeMapping {
    from: u64,
    to: u64,
    len: u64,
}

impl RangeMapping {
    pub fn new(from: u64, to: u64, len: u64) -> Self {
        Self {from, to, len}
    }
}

pub struct RangeMap {
    ranges: Vec<RangeMapping>,
}

impl FromIterator<RangeMapping> for RangeMap {
    fn from_iter<T: IntoIterator<Item = RangeMapping>>(iter: T) -> Self {
        let mut ranges: Vec<_> = iter.into_iter().collect();
        ranges.sort_by_key(|range| range.from);
        Self { ranges }
    }
}

impl RangeMap {
    pub fn get_mappping(&self, from: u64) -> Option<&RangeMapping> {
        match self.ranges.binary_search_by_key(&from, |range| range.from) {
            Ok(i) => Some(&self.ranges[i]),
            Err(i) => {
                if i == 0 {
                    return None;
                }
                let range = &self.ranges[i - 1];
                if from >= range.from + range.len {
                    return None;
                }
                Some(range)
            },
        }
    }

    pub fn get_mappings_slice(&self, range: &Range<u64>) -> &[RangeMapping] {
        let Range { start, end } = range;
        let start = match self.ranges.binary_search_by_key(start, |range| range.from) {
            Ok(i) | Err(i) => i,
        };
        let end = match self.ranges.binary_search_by_key(end, |range| range.from) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        &self.ranges[start..end.min(start)]
    } 

    pub fn get(&self, from: u64) -> u64 {
        let Some(mapping) = self.get_mappping(from) else { return from };
        mapping.to + from - mapping.from
    }

    pub fn get_ranges(&self, from_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut result = vec![];

        for from_range in &from_ranges {
            for mappings in self.get_mappings_slice(from_range) {
                                
            }
        }

        result
    }
}