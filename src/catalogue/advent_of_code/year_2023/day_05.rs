use std::{ops::Range, str::FromStr};

super::get_input!(2023, 5);

super::create_solver!(
    PART_1,
    lowest_location,
    "Advent of Code 2023, Day 5, Part 1. If You Give A Seed A Fertilizer."
);

super::create_solver!(
    PART_2,
    lowest_location_ranged,
    "Advent of Code 2023, Day 5, Part 2. If You Give A Seed A Fertilizer."
);

struct Mapper {
    pub range_orgin: Range<u64>,
    delta: i64,
}

struct AlmanacErr;

impl FromStr for Mapper {
    type Err = AlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: [u64; 3] = s
            .split(' ')
            .filter_map(|num| num.parse().ok())
            .collect::<Vec<u64>>()
            .try_into()
            .map_err(|_| AlmanacErr)?;

        let delta = nums[0] as i64 - nums[1] as i64;
        let start = nums[1];
        let end = nums[1] + nums[2];

        Ok(Self {
            range_orgin: start..end,
            delta,
        })
    }
}

impl Mapper {
    fn map(&self, num: u64) -> Option<u64> {
        if self.range_orgin.contains(&num) {
            Some((num as i64 + self.delta) as u64)
        } else {
            None
        }
    }
}

struct MapperCollection {
    mappers: Vec<Mapper>,
}

impl FromStr for MapperCollection {
    type Err = AlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mappers = s
            .lines()
            .map(Mapper::from_str)
            .collect::<Result<Vec<Mapper>, AlmanacErr>>()?;

        // Se presupone que no se dan mappers solapados.
        mappers.sort_by_key(|e| e.range_orgin.start);

        Ok(Self { mappers })
    }
}

impl MapperCollection {
    fn map(&self, num: u64) -> u64 {
        self.mappers
            .iter()
            .find_map(|mapper| mapper.map(num))
            .unwrap_or(num)
    }

    fn map_range(&self, range: Range<u64>) -> impl IntoIterator<Item = Range<u64>> + '_ {
        MapperCollectionRangeIterator {
            mapper_collection: self,
            range,
            current: 0,
        }
    }
}

struct Almanac {
    seeds: Vec<u64>,
    transformations: [MapperCollection; 7],
}

impl FromStr for Almanac {
    type Err = AlmanacErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<&str>>();

        if parts.len() != 8 {
            // Check input file use \n as line separator and not \r\n
            return Err(AlmanacErr);
        }

        let seeds = parts[0]
            .split_once(':')
            .ok_or(AlmanacErr)?
            .1
            .split_whitespace()
            .map(|num| num.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| AlmanacErr)?;

        let transformations: [MapperCollection; 7] = parts[1..]
            .iter()
            .filter_map(|s| {
                s.split_once(":\n")
                    .ok_or(AlmanacErr)
                    .and_then(|(_, mapper_collection)| {
                        mapper_collection.parse::<MapperCollection>()
                    })
                    .ok()
            })
            .collect::<Vec<MapperCollection>>()
            .try_into()
            .map_err(|_| AlmanacErr)?;

        Ok(Self {
            seeds,
            transformations,
        })
    }
}

impl Almanac {
    fn map(&self, num: u64) -> u64 {
        self.transformations
            .iter()
            .fold(num, |acc, mapper_collection| mapper_collection.map(acc))
    }

    fn find_min_position(&self) -> u64 {
        self.seeds
            .iter()
            .map(|num| self.map(*num))
            .min()
            .unwrap_or(u64::MAX)
    }

    fn find_min_position_range(&self, range: Range<u64>) -> Option<u64> {
        let mut curr_ranges = vec![range];
        let mut next_ranges = vec![];

        for transformation in &self.transformations {
            for range in curr_ranges.drain(..) {
                next_ranges.extend(transformation.map_range(range));
            }
            std::mem::swap(&mut curr_ranges, &mut next_ranges);
        }
        curr_ranges.into_iter().filter_map(|r| r.min()).min()
    }
}

pub fn lowest_location() -> u64 {
    let input = get_input();

    let almanac = Almanac::from_str(&input).unwrap_or_else(|_e| {
        panic!("Error parsing input.");
    });

    almanac.find_min_position()
}

struct MapperCollectionRangeIterator<'a> {
    mapper_collection: &'a MapperCollection,
    range: Range<u64>,
    current: usize,
}

impl<'a> Iterator for MapperCollectionRangeIterator<'a> {
    type Item = Range<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        // No queda nada de procesar en el rango.
        if self.range.end <= self.range.start {
            return None;
        }

        // Iterar por los mappers para encontrar el siguiente aplicable.
        while self.current < self.mapper_collection.mappers.len() {
            let curr_mapper = &self.mapper_collection.mappers[self.current];

            // Si parte inicial del rango está fuera del mapper, no se puede aplicar para dicha parte este ni los siguientes mappers al estar ordenados.
            if self.range.start < curr_mapper.range_orgin.start {
                let start = self.range.start;
                let end = std::cmp::min(self.range.end, curr_mapper.range_orgin.start);
                self.range.start = end;
                return Some(start..end);
            }

            // Si hay overlap entre el rango y el mapper, se aplica el mapper a la parte correspondiente del rango.
            if self.range.start < curr_mapper.range_orgin.end {
                let start = std::cmp::max(self.range.start, curr_mapper.range_orgin.start);
                let end = std::cmp::min(self.range.end, curr_mapper.range_orgin.end);
                self.range.start = end;
                self.current += 1;
                return Some(
                    (start as i64 + curr_mapper.delta) as u64
                        ..(end as i64 + curr_mapper.delta) as u64,
                );
            } else {
                // No hay overpal seguir al siguiente mapper.
                self.current += 1;
            }
        }

        // Si quedo algo del rango por procesar, devolverlo.
        if self.range.end > self.range.start {
            let res = Some(self.range.clone());
            self.range.start = self.range.end;
            return res;
        }

        None
    }
}

pub fn lowest_location_ranged() -> u32 {
    let input = get_input();

    let almanac = Almanac::from_str(&input).unwrap_or_else(|_e| {
        panic!("Error parsing input.");
    });

    almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .filter_map(|range| almanac.find_min_position_range(range))
        .map(|num| num as u32)
        .min()
        .unwrap()
}
