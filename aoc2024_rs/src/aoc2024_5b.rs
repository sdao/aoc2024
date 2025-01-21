use std::collections::HashSet;

use crate::common;

pub fn main()
{
    let lines = common::read_lines("input/5.txt");
    let mut rules = HashSet::<(i32, i32)>::new();
    let mut updates = Vec::<Vec<i32>>::new();
    let mut part = 0i32;
    for line in lines.iter()
    {
        if part == 0
        {
            if line.len() == 0
            {
                part = 1;
            }
            else
            {
                let parts: Vec<&str> = line.split('|').collect();
                let a: i32 = parts[0].parse().unwrap();
                let b: i32 = parts[1].parse().unwrap();
                rules.insert((a, b));
            }
        }
        else
        {
            let parts: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            updates.push(parts);
        }
    }

    for (ra, rb) in &rules
    {
        println!("rule {} | {}", ra, rb);
    }

    let mut sum = 0i32;
    for update in &mut updates
    {
        println!("update {:?}", update);

        let mut ok = false;
        let mut fixed = false;
        while !ok
        {
            ok = true;

            'outer: for i in 0..update.len()
            {
                for j in (i+1)..update.len()
                {
                    let a = update[i];
                    let b = update[j];
                    if rules.contains(&(b, a))
                    {
                        update.swap(i, j);
                        ok = false;
                        fixed = true;
                        break 'outer;
                    }
                }
            }
        }

        println!("{:?}", update);

        if (fixed)
        {
            sum += update[update.len() / 2];
        }
    }

    println!("{}", sum);
}