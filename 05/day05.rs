use std::fmt;
use std::io::{self, BufRead};
use std::fmt::Debug;
// use std::convert::TryInto;


struct SeedRange {
    // [ )
    start: u64,
    stop: u64
}
impl SeedRange {
    fn create(start: u64, stop: u64) -> SeedRange { 
        assert!(start <= stop, "start={} larger than stop={}", start, stop);
        return SeedRange{start: start, stop: stop};
    }
    fn create_len(start: u64, len: u64) -> SeedRange { 
        return SeedRange{start: start, stop: start + len};
    }
    fn set_stop(&mut self, stop: u64) { 
        assert!(stop >= self.start);
        self.stop = stop;
    }
    fn set_start(&mut self, start: u64) {
        assert!(start <= self.stop);
        self.start = start;
    }
    fn update_from_start(&mut self, start: u64) {
        let len = self.stop - self.start;
        self.start = start;
        self.stop = start + len;
    }
    // fn len(&self) -> usize { 
    //     return (self.stop - self.start).try_into().unwrap();
    // }
}
impl fmt::Display for SeedRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {})", self.start, self.stop)
    }
}
struct ConvertRange {
    src_start: u64,
    dst_start: u64,
    len: u64,
}
impl ConvertRange {
    fn contains(&self, x: u64) -> bool {
        return self.src_start <= x && x < self.src_stop();
    }
    fn convert(&self, x: u64) -> Result<u64, &'static str> {
        if !self.contains(x) { return Err("Not in range"); }
        let offset = x - self.src_start;
        // println!("x: {}, source_start: {}, Offset: {}", x, self.src_start, offset);
        return Ok(self.dst_start + offset);
    }
    fn src_stop(&self) -> u64 {
        // used to detect overflow with u32
        let stop = self.src_start.checked_add(self.len);
        if stop.is_none() {
            panic!("Overflow src_stop: start={}, len={}", self.src_start, self.len);
        }
        return stop.unwrap();
    }
    fn dst_stop(&self) -> u64 {
        return self.dst_start + self.len;
    }
}
impl fmt::Display for ConvertRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{[{}, {}) -> [{}, {})}}", self.src_start, self.src_stop(), self.dst_start, self.dst_stop())
    }
}


fn apply_map(sources: &mut Vec<u64>, map: &Vec<ConvertRange>) {
    for i in 0..sources.len() {
        for entry in map {
            if let Ok(converted) = entry.convert(sources[i]) {
                // println!("Convert {} -> {}", sources[i], converted);
                sources[i] = converted;
                break;
            }
        }
    }
}


fn apply_map2(sources: &mut Vec<SeedRange>, map: &Vec<ConvertRange>) {
    // if a range from a map intersects with a seed range, split the seed range
    let mut i = 0;
    while i < sources.len() {
        for entry in map {
            // ==[  ]
            if sources[i].stop <= entry.src_start { continue; } // fully below range
            // [  ]==
            else if entry.src_stop() <= sources[i].start { continue; }  // fully above range
            // println!("SeedRange={} in ConvertRange={}", sources[i], entry);
            // =[= ] or =[==]= 
            if sources[i].start < entry.src_start { 
                sources.push(SeedRange::create(sources[i].start, entry.src_start));
                sources[i].set_start(entry.src_start);
                // println!("Split -> {} + {}", sources[i], sources[sources.len()-1]);
            }
            // [ =]=
            if entry.src_stop() < sources[i].stop {
                sources.push(SeedRange::create(entry.src_stop(), sources[i].stop));
                sources[i].set_stop(entry.src_stop());
                // println!("Split -> {} + {}", sources[i], sources[sources.len()-1]);
            }
            // [==]
            let newstart = entry.convert(sources[i].start).expect("Could not convert seedrange start");
            sources[i].update_from_start(newstart);
            break;  // dont process twice
        }
        i += 1;
    }
}



fn main() {
    // let input = "example.txt";
    let input = "input.txt";
    let mut lines = read_lines(&input);
    let line = lines.next().unwrap().ok().expect("No line found");
    let Some(colon) = line.find(':') else { panic!("Could not find ':'"); };
    let mut target_seeds: Vec<u64> = split_into_numbers::<u64>(&line[colon+2..]).collect();
    // create seed ranges
    let mut target_seed_ranges: Vec<SeedRange> = Vec::new();
    target_seed_ranges.reserve(20);
    for i in (1..target_seeds.len()).step_by(2) {
        target_seed_ranges.push(SeedRange::create_len(target_seeds[i-1], target_seeds[i]));
    }
    // map
    let mut map: Vec<ConvertRange> = Vec::new();
    map.reserve(10);
    for (i, line) in lines.enumerate() {
        let Ok(line) = line else { panic!("Line not ok"); };
        if line.is_empty() { continue; }
        if line.find(':').is_some() {
            if i < 3 { continue; }  // skip first time
            apply_map(&mut target_seeds, &map);
            apply_map2(&mut target_seed_ranges, &map);
            map.clear();
            continue;
        }
        else {
            let mut values = split_into_numbers::<u64>(&line);
            map.push(ConvertRange{ dst_start: values.next().expect("No source_start"), src_start: values.next().expect("No dest_start"), len: values.next().expect("No range_len") });
        }
    }
    apply_map(&mut target_seeds, &map);
    apply_map2(&mut target_seed_ranges, &map);
    println!("Smalles location number (1): {}", target_seeds.iter().min().expect("No min found"));
    println!("Smalles location number (2): {}", target_seed_ranges.iter().map(|range| range.start).min().expect("No min found"));
    // for entry in &target_seed_ranges { println!("   {}", entry); }
}

fn split_into_numbers<T: std::str::FromStr>(x: &str) -> impl Iterator<Item = T> + '_ where <T as std::str::FromStr>::Err: Debug {
    return x.split(' ').filter(|&n| {n != "" && n != " "}).map(|n| n.parse::<T>().unwrap());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}
