#[inline]
pub fn first_part() -> usize {
    let mut buffer = String::with_capacity(2);
    let mut acc = vec![];
    const SPLITTER: [char; 3] = [',', '-', '\n'];

    for c in include_str!("input.txt").chars() {
        if SPLITTER.contains(&c) {
            acc.push(buffer.parse::<u8>().unwrap());
            buffer.clear();
        } else {
            buffer.push(c);
        }
    }

    acc.push(buffer.parse::<u8>().unwrap());

    acc
        .chunks_exact(4)
        .filter(|e| (e[0] >= e[2] && e[1] <= e[3]) || (e[2] >= e[0] && e[3] <= e[1]))
        .count()
}

#[inline]
pub fn second_part() -> usize {
    let mut buffer = String::with_capacity(2);
    let mut acc = vec![];
    const SPLITTER: [char; 3] = [',', '-', '\n'];

    for c in include_str!("input.txt").chars() {
        if SPLITTER.contains(&c) {
            acc.push(buffer.parse::<u8>().unwrap());
            buffer.clear();
        } else {
            buffer.push(c);
        }
    }

    acc.push(buffer.parse::<u8>().unwrap());

    acc
        .chunks_exact(4)
        .filter(|e| e[0].max(e[2]) <= e[1].min(e[3]))
        .count()
}