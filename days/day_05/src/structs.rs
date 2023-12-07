use std::ops::RangeInclusive;

pub struct RangeMapDto {
    from: u64,
    to: u64,
    len: u64,
}

impl RangeMapDto {
    pub fn new(from: u64, to: u64, len: u64) -> Self {
        Self { from, to, len }
    }
}

pub struct RangeMapping {
    from: u64,
    to: u64,
}

impl RangeMapping {
    pub fn new(from: u64, to: u64) -> Self {
        Self { from, to }
    }
}

pub struct RangeMap {
    ranges: Vec<RangeMapping>,
}

impl FromIterator<RangeMapDto> for RangeMap {
    fn from_iter<T: IntoIterator<Item = RangeMapDto>>(iter: T) -> Self {
        let dtos: Vec<_> = iter.into_iter().collect();
        let mut ranges: Vec<_> = dtos
            .iter()
            .map(|elem| RangeMapping::new(elem.from, elem.to))
            .chain(
                dtos.iter()
                    .map(|elem| RangeMapping::new(elem.from + elem.len, elem.from + elem.len)),
            )
            .chain(Some(RangeMapping { from: 0, to: 0 }))
            .collect();

        ranges.sort_by_key(|range| range.from);
        ranges.dedup_by_key(|range| range.from);
        Self { ranges }
    }
}

impl RangeMap {
    fn get_mappping_index(&self, from: u64) -> usize {
        match self.ranges.binary_search_by_key(&from, |range| range.from) {
            Ok(i) => i,
            Err(i) => i - 1,
        }
    }

    pub fn get_mapping(&self, from: u64) -> &RangeMapping {
        &self.ranges[self.get_mappping_index(from)]
    }

    pub fn get(&self, from: u64) -> u64 {
        let mapping = self.get_mapping(from);
        mapping.to + from - mapping.from
    }

    pub fn get_ranges(&self, from_ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
        let mut result = vec![];

        for from_range in &from_ranges {
            let start = from_range.start();
            let end = from_range.end();
            let start_mapping = self.get_mappping_index(*start);
            let end_mapping = self.get_mappping_index(*end);

            for i in start_mapping..=end_mapping {
                let mapped_start = self.ranges[i].to + start.saturating_sub(self.ranges[i].from);
                let mapped_end = if i == end_mapping {
                    self.ranges[i].to + end - self.ranges[i].from
                } else {
                    self.ranges[i].to + self.ranges[i + 1].from - self.ranges[i].from
                };
                result.push(mapped_start..=mapped_end);
            }
        }

        result
    }
}
