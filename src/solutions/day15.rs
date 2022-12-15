use std::cmp::{max, min};

use crate::utils::Reader;

pub fn part1(input_path: &str) {
    let readings = parse_sensor_readings(input_path);

    const LINE_Y: isize = 2000000;

    let mut ranges = Vec::new();
    for reading in readings.iter() {
        let range = mdist(reading.sensor_pos, reading.beacon_pos);
        let (sx, sy) = reading.sensor_pos;

        let dist_to_line = (LINE_Y - sy).abs();
        let radius_on_line = range - dist_to_line;
        if radius_on_line >= 0 {
            ranges.push((sx - radius_on_line, sx + radius_on_line));
        }
    }

    ranges.sort_by_key(|(xmin, _)| *xmin);
    merge_overlapping_ranges(&mut ranges);
    // merge overlapping ranges

    let mut beacons: Vec<Coords> = readings.into_iter().map(|r| r.beacon_pos).collect();
    beacons.sort();
    beacons.dedup();

    let total_length: isize = ranges.iter().map(|(xmin, xmax)| xmax - xmin + 1).sum();
    let num_beacons_in_ranges: isize = beacons
        .iter()
        .filter(|(x, y)| {
            *y == LINE_Y && ranges.iter().any(|(xmin, xmax)| *xmin <= *x && *x <= *xmax)
        })
        .count() as isize;

    let answer = total_length - num_beacons_in_ranges; // - num_beacons;
    println!("{}", answer);
}

/// assumes ranges are sorted by start position
fn merge_overlapping_ranges(ranges: &mut Vec<(isize, isize)>) {
    let mut back = 0;
    let mut front = 1;
    while front < ranges.len() {
        if ranges[front].0 <= ranges[back].1 {
            // todo: fix stuff
            ranges[back].1 = max(ranges[front].1, ranges[back].1);
        } else {
            // advance
            back += 1;
            ranges.swap(front, back);
        }
        front += 1;
    }
    ranges.truncate(back + 1);
}

pub fn part2(input_path: &str) {
    let readings = parse_sensor_readings(input_path);
    let sensors: Vec<Sensor> = readings
        .iter()
        .map(|r| Sensor {
            pos: r.sensor_pos,
            range: mdist(r.sensor_pos, r.beacon_pos),
        })
        .collect();

    let mut edges = Vec::new();
    for sensor in &sensors {
        edges.extend(sensor_edges(sensor));
    }

    // compute alignments
    let mut both_aligned = Vec::new();
    for (i, s1) in edges.iter().enumerate() {
        for s2 in edges[i + 1..].iter() {
            if s1.intercept == s2.intercept
                && s1.slope == s2.slope
                && s1.align == -s2.align
                && ranges_overlap(s1.xmin, s1.xmax, s2.xmin, s2.xmax)
            {
                both_aligned.push(EdgeSegment {
                    intercept: s1.intercept,
                    xmin: max(s1.xmin, s2.xmin),
                    xmax: min(s1.xmax, s2.xmax),
                    slope: s1.slope,
                    align: 0,
                });
            }
        }
    }

    let mut candidate_points = Vec::new();
    for (i, s1) in both_aligned.iter().enumerate() {
        for s2 in both_aligned[i + 1..].iter() {
            if s1.slope == -s2.slope && ranges_overlap(s1.xmin, s1.xmax, s2.xmin, s2.xmax) {
                let x = (s2.intercept - s1.intercept) / (s1.slope - s2.slope);

                if s1.xmin <= x && x <= s1.xmax && s2.xmin <= x && x <= s2.xmax {
                    let y = s1.slope * x + s1.intercept;
                    candidate_points.push((x, y));
                }
            }
        }
    }

    for p in candidate_points {
        if sensors.iter().all(|s| !s.covers(p)) {
            let (x, y) = p;
            let tuning_frequency = 4000000 * x + y;
            println!("({}, {}) -> {}", x, y, tuning_frequency);
        }
    }
}

fn ranges_overlap(start1: isize, end1: isize, start2: isize, end2: isize) -> bool {
    (start1 <= start2 && end1 >= start2) || (start2 <= start1 && end2 >= start1)
}

fn sensor_edges(sensor: &Sensor) -> [EdgeSegment; 4] {
    let (sx, sy) = sensor.pos;
    let range = sensor.range + 1;

    [
        // north-west
        EdgeSegment {
            intercept: sy + range - sx,
            xmin: sx - range,
            xmax: sx,
            align: 1,
            slope: 1,
        },
        //north-east
        EdgeSegment {
            intercept: sy + range + sx,
            xmin: sx,
            xmax: sx + range,
            align: 1,
            slope: -1,
        },
        // south-west
        EdgeSegment {
            intercept: sy - range + sx,
            xmin: sx - range,
            xmax: sx,
            align: -1,
            slope: -1,
        },
        // south-east
        EdgeSegment {
            intercept: sy - range - sx,
            xmin: sx,
            xmax: sx + range,
            align: -1,
            slope: 1,
        },
    ]
}

type Coords = (isize, isize);

#[derive(Debug)]
struct EdgeSegment {
    /// y-intercept for the slope line
    intercept: isize,
    /// start x-coordinate
    xmin: isize,
    /// end x-coordinate (inclusize)
    xmax: isize,
    /// Slope of the edge, -1 or +1
    slope: isize,
    /// what direction the edge 'covers' the sensor range;
    /// -1 for edge is "below", +1 for edge is "on top".
    align: isize,
}

#[derive(Debug)]
struct Sensor {
    pos: Coords,
    range: isize,
}

impl Sensor {
    fn covers(&self, coords: Coords) -> bool {
        mdist(self.pos, coords) <= self.range
    }
}

fn mdist(fst: Coords, snd: Coords) -> isize {
    let (x1, y1) = fst;
    let (x2, y2) = snd;
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[derive(Debug)]
struct SensorReading {
    sensor_pos: Coords,
    beacon_pos: Coords,
}

fn parse_sensor_readings(input_path: &str) -> Vec<SensorReading> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut readings = Vec::new();
    while reader.has_next() {
        let reading = read_sensor_reading(&mut reader);
        readings.push(reading);
    }
    readings
}

fn read_sensor_reading(reader: &mut Reader) -> SensorReading {
    reader.skip_lit(b"Sensor at x=");
    let sensor_x = reader.read_isize();
    reader.skip_lit(b", y=");
    let sensor_y = reader.read_isize();
    reader.skip_lit(b": closest beacon is at x=");
    let beacon_x = reader.read_isize();
    reader.skip_lit(b", y=");
    let beacon_y = reader.read_isize();
    reader.skip_lit(b"\n");
    SensorReading {
        sensor_pos: (sensor_x, sensor_y),
        beacon_pos: (beacon_x, beacon_y),
    }
}
