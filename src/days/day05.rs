use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, multispace0},
    multi::{many1, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::types::*;

pub struct Solver;

#[derive(Debug)]
pub struct SeedMaps {
    seeds: Vec<u64>,
    seed_to_soil: Mapper,
    soil_to_fert: Mapper,
    fert_to_water: Mapper,
    water_to_light: Mapper,
    light_to_temp: Mapper,
    temp_to_hum: Mapper,
    hum_to_loc: Mapper,
}
impl SeedMaps {
    fn seed_to_loc(&self, id: u64) -> u64 {
        let soil = self.seed_to_soil.map(id);
        let fert = self.soil_to_fert.map(soil);
        let water = self.fert_to_water.map(fert);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temp.map(light);
        let hum = self.temp_to_hum.map(temp);
        self.hum_to_loc.map(hum)
    }

    fn loc_to_seed(&self, id: u64) -> Option<u64> {
        let hum = self.hum_to_loc.map_rev(id).unwrap_or(id);
        let temp = self.temp_to_hum.map_rev(hum).unwrap_or(hum);
        let light = self.light_to_temp.map_rev(temp).unwrap_or(temp);
        let water = self.water_to_light.map_rev(light).unwrap_or(light);
        let fert = self.fert_to_water.map_rev(water).unwrap_or(water);
        let soil = self.soil_to_fert.map_rev(fert).unwrap_or(fert);
        self.seed_to_soil.map_rev(soil)
    }
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    out_start: u64,
    in_start: u64,
    len: u64,
}

impl Mapper {
    fn map(&self, input: u64) -> u64 {
        for r in self.ranges.iter() {
            if input < r.in_start {
                continue;
            }
            let offset = input - r.in_start;
            if offset >= r.len {
                continue;
            }
            return r.out_start + offset;
        }
        input
    }
    fn map_rev(&self, input: u64) -> Option<u64> {
        for r in self.ranges.iter() {
            if input < r.out_start {
                continue;
            }
            let offset = input - r.out_start;
            if offset >= r.len {
                continue;
            }
            return Some(r.in_start + offset);
        }
        None
    }
}

impl<'a> DaySolver<'a> for Solver {
    type Input = SeedMaps;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        let (_, parsed) = nom_parse(input).unwrap();
        parsed
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        test_print!(test, "{input:#?}");
        let mut min = u64::MAX;
        for seed in input.seeds.clone() {
            let val = input.seed_to_loc(seed);
            min = std::cmp::min(val, min);
        }
        min.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let seeds = input.seeds.clone();
        let pairs = seeds.chunks(2).collect::<Vec<_>>();
        test_print!(test, "seeds: {}, pairs: {}", input.seeds.len(), pairs.len());
        // get starting max value by using part 1 rules
        let mut max = input
            .seeds
            .iter()
            .map(|seed| input.seed_to_loc(*seed))
            .min()
            .unwrap();
        // iterate over possible locations and find smallest possible seed
        // start with big steps and get smaller after overshooting
        let mut min = 0;
        let mut loc = 0;
        let mut step = 10000;
        loop {
            if loc < max {
                loc += step;
            } else if step == 1 {
                break;
            } else {
                step /= 10;
                loc = min;
            }
            if let Some(seed) = input.loc_to_seed(loc) {
                // check if valid seed
                if pairs
                    .iter()
                    .any(|pair| seed >= pair[0] && seed < pair[0] + pair[1])
                {
                    test_print!(test, "loc: {loc}, seed: {seed}");
                    min = loc - step;
                    max = loc;
                }
            }
        }
        loc.to_string()
    }
}

fn seed_list(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds:")(input)?;
    let (rest, seeds) = delimited(
        multispace0,
        separated_list0(complete::char(' '), complete::u64),
        complete::newline,
    )(input)?;
    Ok((rest, seeds))
}

fn mapper(input: &str) -> IResult<&str, Mapper> {
    let (input, _) = tuple((take_until("map:"), tag("map:\n")))(input)?;
    let (rest, ranges) = many1(range_map)(input)?;
    Ok((rest, Mapper { ranges }))
}

fn range_map(input: &str) -> IResult<&str, Range> {
    let (rest, (out_start, _, in_start, _, len)) = terminated(
        tuple((
            complete::u64,
            complete::space1,
            complete::u64,
            complete::space1,
            complete::u64,
        )),
        complete::newline,
    )(input)?;
    Ok((
        rest,
        Range {
            out_start,
            in_start,
            len,
        },
    ))
}

fn nom_parse(input: &str) -> IResult<&str, SeedMaps> {
    let (
        input,
        (
            seeds,
            seed_to_soil,
            soil_to_fert,
            fert_to_water,
            water_to_light,
            light_to_temp,
            temp_to_hum,
            hum_to_loc,
        ),
    ) = tuple((
        seed_list, mapper, mapper, mapper, mapper, mapper, mapper, mapper,
    ))(input)?;
    Ok((
        input,
        SeedMaps {
            seeds,
            seed_to_soil,
            soil_to_fert,
            fert_to_water,
            water_to_light,
            light_to_temp,
            temp_to_hum,
            hum_to_loc,
        },
    ))
}
