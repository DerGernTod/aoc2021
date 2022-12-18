use std::{collections::HashSet, fs, fmt::Display};

pub fn day_25() {
    let world = parse_into_world("./input/day_25.txt");
    println!("Deadlocked after {} steps!", calc_steps_until_deadlocked(world));
}
// easts, souths, maxima
struct World(HashSet<(usize, usize)>, HashSet<(usize, usize)>, (usize, usize));

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.2.1 {
            for x in 0..self.2.0 {
                if self.0.contains(&(x, y)) {
                    write!(f, ">")?;
                } else if self.1.contains(&(x, y)) {
                    write!(f, "v")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_into_world(path: &str) -> World {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| line
            .char_indices()
            .map(move |(col, ch)| (col, row, ch)))
        .fold(World(HashSet::new(), HashSet::new(), (0, 0)), |World(mut easts, mut souths, mut max), (x, y, ch)| {
            match ch {
                '>' => { easts.insert((x, y)); },
                'v' => { souths.insert((x, y)); },
                _ => ()
            }
            max = (max.0.max(x + 1), max.1.max(y + 1));
            World(easts, souths, max)
        })
}

fn exec_step(world: &World) -> Option<World> {
    let World(easts, souths, max) = world;
    // println!("{world}");
    let (new_easts, east_blocked) = easts.iter().fold((HashSet::new(), 0), |(mut new_easts, mut num_blocked), east| {
        let next = ((east.0 + 1) % max.0, east.1);
        if easts.contains(&next) || souths.contains(&next) {
            num_blocked += 1;
            new_easts.insert(*east);
        } else {
            new_easts.insert(next);
        }
        (new_easts, num_blocked)
    });
    let (new_souths, south_blocked) = souths.iter().fold((HashSet::new(), 0), |(mut new_souths, mut num_blocked), south| {
        let next = (south.0, (south.1 + 1) % max.1);
        if new_easts.contains(&next) || souths.contains(&next) {
            num_blocked += 1;
            new_souths.insert(*south);
        } else {
            new_souths.insert(next);
        }
        (new_souths, num_blocked)
    });
    if east_blocked + south_blocked == new_easts.len() + new_souths.len() {
        None
    } else {
        Some(World(new_easts, new_souths, *max))
    }
}

fn calc_steps_until_deadlocked(world: World) -> usize {
    let mut steps = 0;
    let mut cur_world = Some(world);
    while let Some(world) = cur_world {
        cur_world = exec_step(&world);
        steps += 1;
    }
    steps
}

#[cfg(test)] 
mod tests {
    use super::{parse_into_world, calc_steps_until_deadlocked};

    #[test]
    fn test_part_1() {
        let world = parse_into_world("./input/day_25.test.txt");
        assert_eq!(calc_steps_until_deadlocked(world), 58);
    }
}