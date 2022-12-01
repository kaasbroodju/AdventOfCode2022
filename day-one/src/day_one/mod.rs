#[inline]
pub fn first_part() -> u32 {
    let x = include_str!("input.txt").lines();

    let mut amount = 0;
    let mut highest = 0;

    for x in x {
        match x.parse::<u32>() {
            Ok(calories) => {
                amount += calories
            }
            Err(_) => {
                if amount > highest {
                    highest = amount;
                }
                amount = 0;
            }
        }
    }

    highest
}

#[inline]
pub fn second_part() -> u32 {
    let x = include_str!("input.txt").lines();
    let mut amount = 0;
    let mut amounts = vec![];

    for x in x {
        match x.parse::<u32>() {
            Ok(calories) => {
                amount += calories
            }
            Err(_) => {
                amounts.push(amount);
                amount = 0;
            }
        }
    }

    amounts.sort();

    amounts[amounts.len() - 3..amounts.len()].iter().sum::<u32>()
}