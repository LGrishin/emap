// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// In order to run this single test from the command line:
// $ cargo test --test benchmark -- --nocapture

use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

const CAPACITY: usize = 100;

macro_rules! eval {
    ($map:expr, $total:expr, $capacity:expr) => {{
        let mut sum = 0;
        for _ in 0..$total {
            $map.clear();
            $map.insert(0, 42);
            for i in 1..$capacity - 1 {
                $map.insert(i, i as i64);
                let v = std::hint::black_box(*$map.get(&i).unwrap());
                assert_eq!(v, i as i64);
            }
            for i in 1..$capacity - 1 {
                $map.remove(&i);
            }
            if $map.iter().find(|(_k, v)| **v == 0).is_some() {
                $map.clear();
            }
            let p = std::hint::black_box($map.iter().find(|(_k, v)| **v == 42).unwrap().1);
            sum += p
        }
        std::hint::black_box(sum)
    }};
}

macro_rules! insert {
    ($name:expr, $ret:expr, $map:expr, $total:expr) => {{
        let start = Instant::now();
        let mut m = $map;
        eval!(m, $total, CAPACITY);
        let e = start.elapsed();
        $ret.insert($name, e);
    }};
}

fn benchmark(total: usize) -> HashMap<&'static str, Duration> {
    let mut ret = HashMap::new();
    insert!(
        "std::collections::HashMap",
        ret,
        HashMap::<usize, i64>::with_capacity(CAPACITY),
        total
    );
    insert!(
        "hashbrown::HashMap",
        ret,
        hashbrown::HashMap::<usize, i64>::new(),
        total
    );
    insert!(
        "rustc_hash::FxHashMap",
        ret,
        rustc_hash::FxHashMap::<usize, i64>::default(),
        total
    );
    insert!(
        "nohash_hasher::BuildNoHashHasher",
        ret,
        HashMap::<usize, i64, nohash_hasher::BuildNoHashHasher<u32>>::with_capacity_and_hasher(
            2,
            nohash_hasher::BuildNoHashHasher::default()
        ),
        total
    );
    insert!(
        "std::collections::BTreeMap",
        ret,
        std::collections::BTreeMap::<usize, i64>::new(),
        total
    );
    insert!(
        "tinymap::array_map::ArrayMap",
        ret,
        tinymap::array_map::ArrayMap::<usize, i64, CAPACITY>::new(),
        total
    );
    insert!(
        "linked_hash_map::LinkedHashMap",
        ret,
        linked_hash_map::LinkedHashMap::<usize, i64>::new(),
        total
    );
    insert!(
        "linear_map::LinearMap",
        ret,
        linear_map::LinearMap::<usize, i64>::new(),
        total
    );
    insert!(
        "indexmap::IndexMap",
        ret,
        indexmap::IndexMap::<usize, i64>::new(),
        total
    );
    insert!(
        "litemap::LiteMap",
        ret,
        litemap::LiteMap::<usize, i64>::new(),
        total
    );
    insert!(
        "micromap::Map",
        ret,
        micromap::Map::<usize, i64, CAPACITY>::new(),
        total
    );
    insert!("emap::Map", ret, emap::Map::<i64, CAPACITY>::new(), total);
    ret
}

#[test]
pub fn benchmark_and_print() {
    let times = benchmark(1000);
    let ours = times.get("emap::Map").unwrap();
    for (m, d) in &times {
        println!(
            "{m} -> {:?} ({:.2}x)",
            d,
            d.as_nanos() as f64 / ours.as_nanos() as f64
        );
        if d == ours {
            continue;
        }
        assert!(d.cmp(ours).is_gt());
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let times = benchmark(args.get(1).unwrap().parse::<usize>().unwrap());
    let mut lines = vec![];
    for (m, d) in &times {
        lines.push(format!("{m}\t{}", d.as_nanos()));
    }
    lines.sort();
    for t in lines {
        println!("{t}");
    }
}
