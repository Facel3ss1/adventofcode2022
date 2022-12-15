use std::collections::HashSet;
use std::ops::RangeInclusive;

struct Sensor {
    x: i32,
    y: i32,
    radius: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct Beacon {
    x: i32,
    y: i32,
}

impl Sensor {
    fn row_intersection(&self, y: i32) -> Option<RangeInclusive<i32>> {
        (self.y.abs_diff(y) <= self.radius).then(|| {
            let radius = (self.radius - self.y.abs_diff(y)) as i32;
            self.x - radius..=self.x + radius
        })
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
        .iter()
        .filter(|b| b.y == 2_000_000)
        .map(|beacon| beacon.x)
        .collect::<HashSet<i32>>();

    let row = sensors
        .iter()
        .flat_map(|s| s.row_intersection(2_000_000))
        .flatten()
        .filter(|x| !beacons_on_row.contains(x))
        .collect::<HashSet<i32>>();

    println!("Part 1: {}", row.len());
}
