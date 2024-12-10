use std::fs;
use std::collections::HashSet;
use std::ops::Range;

#[derive(Clone, Debug)]
struct Segment {
    val: Option<usize>,
    size: usize
}

fn defrag_1(mut raw: Vec<usize>) -> usize {
    let fs_size: usize = raw.clone().into_iter().step_by(2).sum();
    let mut fs_repr: Vec<usize> = Vec::new();
    fs_repr.reserve_exact(fs_size);

    // key insight: we iterate from the front and alternately with ids from the front and back. we
    // stop when the indices overlap.
    let mut f_idx = 0;
    let mut b_idx = raw.len() - 1;
    let mut f_file = 0;
    let mut b_file = (raw.len() - 1) / 2;
    while f_idx <= b_idx {
        // sanity check
        assert_eq!(f_idx % 2, 0);
        assert_eq!(b_idx % 2, 0);

        // push the occupied blocks
        for _i in 0..raw[f_idx] {
            fs_repr.push(f_file);
        }
        f_file += 1;
        f_idx += 1;

        // draw from the occupied blocks at the back to fill free at the front
        'outer: for _i in 0..raw[f_idx] {
            // current file from the back is fully defragged
            if raw[b_idx] <= 0 {
                b_idx -= 2;
                b_file -= 1;
            }
            // just in case we broke our loop condition here
            if b_idx <= f_idx {
                break 'outer;
            }
            raw[b_idx] -= 1;
            fs_repr.push(b_file);
        }
        f_idx += 1;
    }
    assert_eq!(fs_size, fs_repr.len());

    let checksum: usize = fs_repr.iter().enumerate().map(|(i, f)| i * f).sum();
    checksum
}

fn defrag_2(raw: Vec<usize>) -> usize {
    assert_eq!(raw.len() % 2, 1);
    let last_fid = (raw.len() - 1) / 2;
    // compressed filesystem representation
    let mut fs: Vec<Segment> = Vec::new();

    for (i, ss) in raw.iter().enumerate() {
        if i % 2 == 0 {
            fs.push(Segment{val: Some(i / 2), size: *ss});
        } else {
            fs.push(Segment{val: None, size: *ss});
        }
    }

    for fid in (0..=last_fid).rev() {
        for i in 0..fs.len() {
            if fs[i].val == Some(fid) {
                let file = fs[i].clone();
                for j in 0..i {
                    let gap = &fs[j];
                    match gap.val {
                        Some(v) => {
                            if v == file.val.unwrap() {
                                break;
                            }
                        },
                        None => {
                            if gap.size >= file.size {
                                let diff = gap.size - file.size;
                                // replace in earlier location. Note that i > j always.
                                fs.remove(j);
                                fs.insert(j, file);
                                if diff > 0 {
                                    fs.insert(j + 1, Segment{val: None, size: diff});
                                }
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // then clean out the back
    let mut seen: HashSet<usize> = HashSet::new();
    let mut idx = 0;
    while idx < fs.len() {
        let mut f = &mut fs[idx];
        match f.val {
            Some(v) => {
                if seen.contains(&v) {
                    f.val = None;
                } else {
                    seen.insert(v);
                    idx += 1;
                }
            },
            None => {idx += 1;}
        };
    }

    let mut fs_repr: Vec<usize> = Vec::new();
    fs_repr.reserve_exact(raw.iter().sum());

    for f in fs {
        for s in 0..f.size {
            match f.val {
                Some(v) => fs_repr.push(v),
                None => fs_repr.push(0)
            };
        }
    }

    let checksum: usize = fs_repr.iter().enumerate().map(|(i, f)| i * f).sum();
    checksum
}

pub fn main(file: &str) {
    let f = fs::read_to_string(file).unwrap();
    let raw: Vec<_> = f.trim().chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    // it wouldn't make a great deal of sense to show the free blocks at the end, so I'm assuming
    // the filesystem always ends in an occupied block, but let's make sure so I don't break my
    // assumptions later...
    assert_eq!(raw.len() % 2, 1);

    let checksum = vec![defrag_1(raw.clone()), defrag_2(raw)];

    println!("Part 1 checksum: {}", checksum[0]);
    println!("Part 2 checksum: {}", checksum[1]);
}
