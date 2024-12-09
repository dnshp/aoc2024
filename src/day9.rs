use std::fs;

pub fn main(file: &str) {
    let f = fs::read_to_string(file).unwrap();
    let mut raw: Vec<_> = f.trim().chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    // it wouldn't make a great deal of sense to show the free blocks at the end, so I'm assuming
    // the filesystem always ends in an occupied block, but let's make sure so I don't break my
    // assumptions later...
    assert_eq!(raw.len() % 2, 1);

    let fs_size: usize = raw.clone().into_iter().step_by(2).sum();
    let mut fs_repr: Vec<usize> = Vec::new();
    fs_repr.reserve_exact(fs_size);

    // key insight: we iterate from the front and alternately with ids from the front and back. we
    // stop when the indices overlap.
    let mut f_idx = 0;
    let mut b_idx = raw.len() - 1;
    let mut f_file = 0;
    let mut b_file = (raw.len() - 1) / 2;
    while f_idx < b_idx {
        // sanity check
        assert_eq!(f_idx % 2, 0);
        assert_eq!(b_idx % 2, 0);
        for i in 0..raw[f_idx] {
            fs_repr.push(f_file);
        }
        f_file += 1;
        f_idx += 1;

        'outer: for i in 0..raw[f_idx] {
            if raw[b_idx] <= 0 {
                b_idx -= 2;
                b_file -= 1;
            }
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

    println!("{}", checksum);
}
