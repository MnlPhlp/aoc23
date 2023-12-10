use nom::{
    bytes::complete::{tag, take_until},
    character::{
        self,
        complete::{self, multispace0},
    },
    multi::{count, many1, separated_list0},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::types::*;

pub struct Solver;

#[derive(Debug)]
pub struct SeedMaps {
    seeds: Vec<u32>,
    seed_to_soil: Mapper,
    soil_to_fert: Mapper,
    fert_to_water: Mapper,
    water_to_light: Mapper,
    light_to_temp: Mapper,
    temp_to_hum: Mapper,
    hum_to_loc: Mapper,
}
impl SeedMaps {
    fn seed_to_loc(&self, id: u32) -> u32 {
        let soil = self.seed_to_soil.map(id);
        let fert = self.soil_to_fert.map(soil);
        let water = self.fert_to_water.map(fert);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temp.map(light);
        let hum = self.temp_to_hum.map(temp);
        self.hum_to_loc.map(hum)
    }
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    out_start: u32,
    in_start: u32,
    len: u32,
}

impl Mapper {
    fn map(&self, input: u32) -> u32 {
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
}

impl<'a> DaySolver<'a> for Solver {
    type Input = SeedMaps;

    fn parse_input(input: &'a str) -> Self::Input {
        let (_, parsed) = nom_parse(input).unwrap();
        parsed
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        test_print!(test, "{input:#?}");
        let min = input
            .seeds
            .iter()
            .map(|seed| input.seed_to_loc(*seed))
            .min()
            .unwrap();
        min.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let pairs = input.seeds.chunks(2);
        test_print!(test, "seeds: {}, pairs: {}", input.seeds.len(), pairs.len());
        let min = pairs
            .map(|pair| {
                let start = pair[0];
                let count = pair[1];
                (start..start + count)
                    .map(|seed| input.seed_to_loc(seed))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap();
        min.to_string()
    }
}

fn seed_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds:")(input)?;
    let (rest, seeds) = delimited(
        multispace0,
        separated_list0(complete::char(' '), complete::u32),
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
            complete::u32,
            complete::space1,
            complete::u32,
            complete::space1,
            complete::u32,
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
