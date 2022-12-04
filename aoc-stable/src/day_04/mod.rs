#[inline]
pub fn first_part() -> u32 {
    include_str!("input.txt")
        .split(|c| {(c == ',' || c == '-' || c == '\n')})
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<_>>()
        .chunks(4)
        .filter(|e| {
            (e[0] >= e[2] && e[1] <= e[3]) || (e[2] >= e[0] && e[3] <= e[1])
        })
        .count() as u32
}

#[inline]
pub fn second_part() -> u32 {
    include_str!("input.txt")
        .split(|c| {(c == ',' || c == '-' || c == '\n')})
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<_>>()
        .chunks(4)
        .filter(|e| {
            e[0].max(e[2]) <= e[1].min(e[3])
        })
        .count() as u32
}