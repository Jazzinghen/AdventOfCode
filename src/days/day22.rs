use itertools::Itertools;
use nalgebra::Point3;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

use std::convert::TryFrom;

fn power(input: &str) -> IResult<&str, bool> {
    let (rem_str, power) = alt((tag("on"), tag("off")))(input)?;

    Ok((rem_str, power == "on"))
}

fn axis_range(input: &str) -> IResult<&str, (i32, i32)> {
    let (rem_str, (first_raw, second_raw)) = preceded(
        alt((tag("x="), tag("y="), tag("z="))),
        separated_pair(
            pair(opt(tag("-")), digit1),
            tag(".."),
            pair(opt(tag("-")), digit1),
        ),
    )(input)?;

    let first_value = format!("{}{}", first_raw.0.unwrap_or(""), first_raw.1);
    let second_value = format!("{}{}", second_raw.0.unwrap_or(""), second_raw.1);

    Ok((
        rem_str,
        (first_value.parse().unwrap(), second_value.parse().unwrap()),
    ))
}

pub fn power_cube(input: &str) -> IResult<&str, PowerCuboid> {
    let (rem_str, power_state) = delimited(space0, power, space0)(input)?;

    let (rem_str, axes) = separated_list1(tag(","), axis_range)(rem_str)?;

    let x_range = (axes[0].0.min(axes[0].1), axes[0].0.max(axes[0].1) + 1);
    let y_range = (axes[1].0.min(axes[1].1), axes[1].0.max(axes[1].1) + 1);
    let z_range = (axes[2].0.min(axes[2].1), axes[2].0.max(axes[2].1) + 1);

    Ok((
        rem_str,
        PowerCuboid::new(power_state, x_range, y_range, z_range),
    ))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cuboid {
    top_right: Point3<i32>,
    bottom_left: Point3<i32>,
}

impl Cuboid {
    pub fn new(bottom_left: Point3<i32>, top_right: Point3<i32>) -> Self {
        Self {
            top_right,
            bottom_left,
        }
    }

    pub fn inside_volume(&self, volume: &Cuboid) -> bool {
        self.bottom_left >= volume.bottom_left && self.top_right <= volume.top_right
    }

    pub fn volume(&self) -> u64 {
        let sizes = (self.top_right - self.bottom_left).abs();

        sizes
            .into_iter()
            .map(|length| u64::try_from(*length).unwrap())
            .product()
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        if self.bottom_left.x > other.top_right.x || self.top_right.x < other.bottom_left.x {
            return None;
        }
        if self.bottom_left.y > other.top_right.y || self.top_right.y < other.bottom_left.y {
            return None;
        }
        if self.bottom_left.z > other.top_right.z || self.top_right.z < other.bottom_left.z {
            return None;
        }

        let (min_x, max_x) = (
            self.bottom_left.x.max(other.bottom_left.x),
            self.top_right.x.min(other.top_right.x),
        );
        let (min_y, max_y) = (
            self.bottom_left.y.max(other.bottom_left.y),
            self.top_right.y.min(other.top_right.y),
        );
        let (min_z, max_z) = (
            self.bottom_left.z.max(other.bottom_left.z),
            self.top_right.z.min(other.top_right.z),
        );

        Some(Self {
            bottom_left: Point3::new(min_x, min_y, min_z),
            top_right: Point3::new(max_x, max_y, max_z),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PowerCuboid {
    cuboid: Cuboid,
    power_state: bool,
}

impl PowerCuboid {
    fn new(
        power_state: bool,
        x_range: (i32, i32),
        y_range: (i32, i32),
        z_range: (i32, i32),
    ) -> Self {
        let bottom_left: Point3<i32> = Point3::new(x_range.0, y_range.0, z_range.0);
        let top_right: Point3<i32> = Point3::new(x_range.1, y_range.1, z_range.1);

        Self {
            cuboid: Cuboid {
                top_right,
                bottom_left,
            },
            power_state,
        }
    }

    pub fn inside_volume(&self, volume: &Cuboid) -> bool {
        self.cuboid.inside_volume(volume)
    }

    pub fn intersect(&self, other: &Self) -> Option<PowerCuboid> {
        let intersection_cuboid = self.cuboid.intersect(&other.cuboid)?;

        Some(PowerCuboid {
            cuboid: intersection_cuboid,
            power_state: other.power_state,
        })
    }

    fn compute_on_volume(&self, other_cuboids: &[PowerCuboid]) -> u64 {
        let conflicts = other_cuboids
            .iter()
            .filter_map(|c| self.intersect(c))
            .collect_vec();

        let confict_volume: u64 = conflicts
            .iter()
            .enumerate()
            .map(|(idx, cube)| cube.compute_on_volume(&conflicts[idx + 1..]))
            .sum();

        self.cuboid.volume().checked_sub(confict_volume).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parsing() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let ref_cubes = vec![
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(13, 13, 13),
                    bottom_left: Point3::new(10, 10, 10),
                },
                power_state: true,
            },
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(14, 14, 14),
                    bottom_left: Point3::new(11, 11, 11),
                },
                power_state: true,
            },
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(12, 12, 12),
                    bottom_left: Point3::new(9, 9, 9),
                },
                power_state: false,
            },
            PowerCuboid {
                cuboid: Cuboid {
                    top_right: Point3::new(11, 11, 11),
                    bottom_left: Point3::new(10, 10, 10),
                },
                power_state: true,
            },
        ];

        assert_eq!(cubes, ref_cubes);
    }

    #[test]
    fn negative_volume() {
        let test_cuboid = Cuboid::new(Point3::new(-12, -12, -12), Point3::new(-9, -9, -9));

        assert_eq!(test_cuboid.volume(), 27);
    }

    #[test]
    fn crossover_volume() {
        let test_cuboid = Cuboid::new(Point3::new(-3, -3, -3), Point3::new(2, 2, 2));

        assert_eq!(test_cuboid.volume(), 125);
    }

    #[test]
    fn basic_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let intersection = cubes[0].intersect(&cubes[1]).unwrap();

        let ref_intersection = PowerCuboid {
            cuboid: Cuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(11, 11, 11),
            },
            power_state: true,
        };

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(intersection, ref_intersection);
        assert_eq!(final_volume, 46);
    }

    #[test]
    fn no_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let far_cube = PowerCuboid {
            cuboid: Cuboid {
                top_right: Point3::new(16, 16, 16),
                bottom_left: Point3::new(15, 15, 15),
            },
            power_state: true,
        };

        assert!(cubes[0].intersect(&far_cube).is_none());
    }

    #[test]
    fn self_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let intersection = cubes[0].intersect(&cubes[0]).unwrap();

        assert_eq!(intersection, cubes[0]);
        assert_eq!(intersection.cuboid.volume(), 27);
    }

    #[test]
    fn power_switch_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        off x=11..13,y=11..13,z=11..13";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let intersection = cubes[0].intersect(&cubes[1]).unwrap();

        let ref_intersection = PowerCuboid {
            cuboid: Cuboid {
                top_right: Point3::new(13, 13, 13),
                bottom_left: Point3::new(11, 11, 11),
            },
            power_state: false,
        };

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(intersection, ref_intersection);
        assert_eq!(final_volume, 19);
    }

    #[test]
    fn tri_intersection() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        off x=11..13,y=11..13,z=11..13
        on x=12..14,y=10..12,z=10..12";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 41);
    }

    #[test]
    fn longer_test() {
        let input_string = "on x=10..12,y=10..12,z=10..12
        on x=11..13,y=11..13,z=11..13
        off x=9..11,y=9..11,z=9..11
        on x=10..10,y=10..10,z=10..10";

        let cubes: Vec<PowerCuboid> = input_string
            .lines()
            .map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                cube
            })
            .collect();

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 39);
    }

    #[test]
    fn full_centre_power_cycle() {
        let input_string = "on x=-20..26,y=-36..17,z=-47..7
        on x=-20..33,y=-21..23,z=-26..28
        on x=-22..28,y=-29..23,z=-38..16
        on x=-46..7,y=-6..46,z=-50..-1
        on x=-49..1,y=-3..46,z=-24..28
        on x=2..47,y=-22..22,z=-23..27
        on x=-27..23,y=-28..26,z=-21..29
        on x=-39..5,y=-6..47,z=-3..44
        on x=-30..21,y=-8..43,z=-13..34
        on x=-22..26,y=-27..20,z=-29..19
        off x=-48..-32,y=26..41,z=-47..-37
        on x=-12..35,y=6..50,z=-50..-2
        off x=-48..-32,y=-32..-16,z=-15..-5
        on x=-18..26,y=-33..15,z=-7..46
        off x=-40..-22,y=-38..-28,z=23..41
        on x=-16..35,y=-41..10,z=-47..6
        off x=-32..-23,y=11..30,z=-14..3
        on x=-49..-5,y=-3..45,z=-29..18
        off x=18..30,y=-20..-8,z=-3..13
        on x=-41..9,y=-7..43,z=-33..15
        on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        on x=967..23432,y=45373..81175,z=27513..53682";

        let target_volume = Cuboid::new(Point3::new(-50, -50, -50), Point3::new(51, 51, 51));

        let cubes = input_string
            .lines()
            .filter_map(|line| {
                let (_, cube) = power_cube(line).unwrap();
                if cube.inside_volume(&target_volume) {
                    Some(cube)
                } else {
                    None
                }
            })
            .collect_vec();

        assert_eq!(cubes.len(), 20);

        let final_volume: u64 = cubes
            .iter()
            .enumerate()
            .filter(|(_, c)| c.power_state)
            .map(|(idx, c)| c.compute_on_volume(&cubes[idx + 1..]))
            .sum();

        assert_eq!(final_volume, 590784);
    }
}
