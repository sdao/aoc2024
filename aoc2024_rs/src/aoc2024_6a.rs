
fn advance(table: &mut Vec<Vec<char>>) -> bool
{
    let height = table.len();
    let width = table[0].len();

    for i in 0..height
    {
        for j in 0..width
        {
            match table[i][j] {
                '^' => {
                    if i == 0
                    {
                        table[i][j] = 'X';
                        return false;
                    }
                    else
                    {
                        let target = table[i-1][j];
                        if target == '#'
                        {
                            table[i][j] = '>';
                        }
                        else
                        {
                            table[i-1][j] = '^';
                            table[i][j] = 'X';
                        }
                        return true;
                    }
                },
                '<' => {
                    if j == 0
                    {
                        table[i][j] = 'X';
                        return false;
                    }
                    else
                    {
                        let target = table[i][j-1];
                        if target == '#'
                        {
                            table[i][j] = '^';
                        }
                        else
                        {
                            table[i][j-1] = '<';
                            table[i][j] = 'X'; 
                        }
                        return true;
                    }
                },
                '>' => {
                    if j == width-1
                    {
                        table[i][j] = 'X';
                        return false;
                    }
                    else {
                        let target = table[i][j+1];
                        if target == '#'
                        {
                            table[i][j] = 'v';
                        }
                        else {
                            table[i][j+1] = '>';
                            table[i][j] = 'X';
                        }
                        return true;
                    }
                },
                'v' => {
                    if i == height-1
                    {
                        table[i][j] = 'X';
                        return false;
                    }
                    else
                    {
                        let target = table[i+1][j];
                        if target == '#'
                        {
                            table[i][j] = '<';
                        }
                        else
                        {
                            table[i+1][j] = 'v';
                            table[i][j] = 'X';
                        }
                        return true;
                    }
                },
                _ => {},
            }
        }
    }

    panic!();
}

fn count(table: &Vec<Vec<char>>) -> usize
{
    let mut count_x = 0;
    for line in table
    {
        for c in line
        {
            if *c == 'X'
            {
                count_x += 1;
            }
        }
    }

    count_x
}

pub fn main(input: Vec<String>)
{
    let mut table: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    while advance(&mut table)
    {
    }

    println!("{}", count(&table));
}