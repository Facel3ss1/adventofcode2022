use std::collections::HashSet;
use std::ops::RangeInclusive;

struct Sensor {
    x: i32,
    y: i32,
    radius: u32,
}

impl Sensor {
    fn row_intersection(&self, y: i32) -> Option<RangeInclusive<i32>> {
        (self.y.abs_diff(y) <= self.radius).then(|| {
            let radius = (self.radius - self.y.abs_diff(y)) as i32;
            self.x - radius..=self.x + radius
        })
    }

    fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        let radius = (self.radius + 1) as i32;
        [self.y + radius, self.y - radius]
            .into_iter()
            .flat_map(|y| {
                [Gradient::Positive, Gradient::Negative]
                    .into_iter()
                    .map(move |g| (y, g))
            })
            .map(|(y, g)| {
                let intercept = y + -(g as i32) * self.x;
                Line {
                    gradient: g,
                    intercept,
                }
            })
    }

    fn contains(&self, point: (i32, i32)) -> bool {
        self.x.abs_diff(point.0) + self.y.abs_diff(point.1) <= self.radius
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
enum Gradient {
    Positive = 1,
    Negative = -1,
}

#[derive(PartialEq, Eq, Hash)]
struct Line {
    gradient: Gradient,
    intercept: i32,
}

impl Line {
    // Assumes the other line has the opposite gradient to this line
    fn intersection(&self, other: &Self) -> (i32, i32) {
        let y = (self.intercept + other.intercept) / 2;
        let distance = self.intercept.abs_diff(other.intercept) as i32 / 2;

        let x = match self.intercept.cmp(&other.intercept) {
            std::cmp::Ordering::Less => self.gradient as i32 * distance,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => other.gradient as i32 * distance,
        };

        (x, y)
    }
}

fn parse_position(input: &str) -> (i32, i32) {
    let (x, y) = input.split_once(", ").unwrap();
    let x = x.strip_prefix("x=").unwrap();
    let y = y.strip_prefix("y=").unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

fn main() {
    let (sensors, beacons): (Vec<Sensor>, HashSet<Beacon>) = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (sensor_position, beacon_position) = line.split_once(": ").unwrap();
            let sensor_position = sensor_position.strip_prefix("Sensor at ").unwrap();
            let beacon_position = beacon_position
                .strip_prefix("closest beacon is at ")
                .unwrap();

            let (x, y) = parse_position(sensor_position);
            let (beacon_x, beacon_y) = parse_position(beacon_position);

            let radius = x.abs_diff(beacon_x) + y.abs_diff(beacon_y);

            (
                Sensor { x, y, radius },
                Beacon {
                    x: beacon_x,
                    y: beacon_y,
                },
            )
        })
        .unzip();

    let beacons_on_row = beacons
        .into_iter()
        .filter(|b| b.y == 2_000_000)
        .map(|beacon| beacon.x)
        .collect::<HashSet<i32>>();

    let row = &sensors
        .iter()
        .flat_map(|s| s.row_intersection(2_000_000))
        .flatten()
        .collect::<HashSet<i32>>()
        - &beacons_on_row;

    println!("Part 1: {}", row.len());

    let (positive_lines, negative_lines): (HashSet<Line>, HashSet<Line>) = sensors
        .iter()
        .flat_map(|sensor| sensor.lines())
        .partition(|line| line.gradient == Gradient::Positive);

    let coord_range = 0..=4_000_000;
    let points = positive_lines
        .iter()
        .flat_map(|pos| negative_lines.iter().map(move |neg| pos.intersection(neg)))
        .filter(|point| coord_range.contains(&point.0) && coord_range.contains(&point.1))
        .collect::<HashSet<(i32, i32)>>();

    let (x, y) = points
        .into_iter()
        .find(|&point| sensors.iter().all(|sensor| !sensor.contains(point)))
        .unwrap();

    let tuning_frequency = (x as u64) * 4_000_000 + (y as u64);
    println!("Part 2: {tuning_frequency}");
}
