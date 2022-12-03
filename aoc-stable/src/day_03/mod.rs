

#[inline]
pub fn first_part() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (rug_a, rug_b) = line.split_at(line.len() / 2);
            for a in rug_a.chars() {
                for b in rug_b.chars() {
                    if a == b {
                        return calc_amount(a)
                    }
                }
            }
            1
        })
        .sum()
}

fn calc_amount(a: char) -> u32 {
    let result = a as u32;
    if result > 96 {
        (a as u32 - 96)
    } else {
        (a as u32 - 64) + 26
    }

}

#[inline]
pub fn second_part() -> u32 {
    let x =include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut amounts = vec![];
    for i in (0..x.len()).step_by(3) {
        let (bag_a, bag_b, bag_c) = (x[i], x[i+1], x[i+2]);
        'search_loop: for a in bag_a.chars() {
            for b in bag_b.chars() {
                for c in bag_c.chars() {
                    if a == b && b == c{
                        amounts.push(calc_amount(a));
                        break 'search_loop;
                    }
                }
            }
        }
    }
    amounts.iter().sum()
}