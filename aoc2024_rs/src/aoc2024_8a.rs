use std::collections::{HashMap, HashSet};

pub fn main(inputs: Vec<String>)
{
    let height = inputs.len() as i32;
    let width = inputs[0].len() as i32;

    let mut antennas = HashMap::<char, Vec<(i32, i32)>>::new();
    for (row, line) in inputs.iter().enumerate()
    {
        for (col, c) in line.chars().enumerate()
        {
            if c != '.'
            {
                if let Some(x) = antennas.get_mut(&c)
                {
                    x.push((row as i32, col as i32));
                }
                else
                {
                    antennas.insert(c, vec![(row as i32, col as i32)]);
                }
            }
        }
    }

    println!("{:?}", antennas);

    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (_c, positions) in antennas
    {
        for (i, pa) in positions.iter().enumerate()
        {
            for (j, pb) in positions.iter().enumerate()
            {
                if i != j
                {
                    let drow = pb.0 - pa.0;
                    let dcol = pb.1 - pa.1;
                    
                    let mut row = pb.0;
                    let mut col = pb.1;

                    while row >= 0 && row < height && col >=0 && col < width
                    {
                        antinodes.insert((row, col));

                        row += drow;
                        col += dcol;
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}