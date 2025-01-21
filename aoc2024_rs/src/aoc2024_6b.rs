#[derive(Debug)]
enum Movement
{
    Normal,
    Escaped,
    Looped,
}

fn advance(table: &mut Vec<Vec<char>>, visits: &mut Vec<Vec<[bool; 4]>>) -> Movement
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
                        return Movement::Escaped;
                    }
                    else
                    {
                        let target = table[i-1][j];
                        if target == '#'
                        {
                            table[i][j] = '>';
                            return Movement::Normal;
                        }
                        else
                        {
                            table[i-1][j] = '^';
                            table[i][j] = 'X';

                            let chk = check_loop(visits[i-1][j], 0);
                            visits[i-1][j][0] = true;
                            return chk;
                        }
                    }
                },
                '<' => {
                    if j == 0
                    {
                        table[i][j] = 'X';
                        return Movement::Escaped;
                    }
                    else
                    {
                        let target = table[i][j-1];
                        if target == '#'
                        {
                            table[i][j] = '^';
                            return Movement::Normal;
                        }
                        else
                        {
                            table[i][j-1] = '<';
                            table[i][j] = 'X'; 

                            let chk = check_loop(visits[i][j-1], 1);
                            visits[i][j-1][1] = true;
                            return chk;
                        }
                    }
                },
                '>' => {
                    if j == width-1
                    {
                        table[i][j] = 'X';
                        return Movement::Escaped;
                    }
                    else {
                        let target = table[i][j+1];
                        if target == '#'
                        {
                            table[i][j] = 'v';
                            return Movement::Normal;
                        }
                        else {
                            table[i][j+1] = '>';
                            table[i][j] = 'X';

                            let chk = check_loop(visits[i][j+1], 2);
                            visits[i][j+1][2] = true;
                            return chk;
                        }
                    }
                },
                'v' => {
                    if i == height-1
                    {
                        table[i][j] = 'X';
                        return Movement::Escaped;
                    }
                    else
                    {
                        let target = table[i+1][j];
                        if target == '#'
                        {
                            table[i][j] = '<';
                            return Movement::Normal;
                        }
                        else
                        {
                            table[i+1][j] = 'v';
                            table[i][j] = 'X';

                            let chk = check_loop(visits[i+1][j], 3);
                            visits[i+1][j][3] = true;
                            return chk;
                        }
                    }
                },
                _ => {},
            }
        }
    }

    panic!();
}

fn check_loop(v: [bool; 4], dir: usize) -> Movement
{
    if v[dir]
    {
        Movement::Looped
    }
    else {
        Movement::Normal
    }
}

fn simulate(table: &mut Vec<Vec<char>>, visits: &mut Vec<Vec<[bool; 4]>>) -> Movement
{
    loop
    {
        match advance(table, visits)
        {
            Movement::Escaped => return Movement::Escaped,
            Movement::Looped => return Movement::Looped,
            _ => {}//print_table(&table),
        }
    }
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

fn print_table(table: &Vec<Vec<char>>)
{
    for line in table
    {
        for c in line
        {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub fn main(input: Vec<String>)
{
    let table: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let visits: Vec<Vec<[bool; 4]>> = input.iter().map(|line| line.chars().map(|c| match c {
        '^' => [true, false, false, false],
        '<' => [false, true, false, false],
        '>' => [false, false, true, false],
        'v' => [false, false, false, true],
        _ => [false, false, false, false],
    }).collect()).collect();

    let height = table.len();
    let width = table[0].len();

    let mut combos = 0;
    for i in 0..height
    {
        for j in 0..width
        {
            if table[i][j] == '.'
            {
                let mut table_clone = table.clone();
                let mut visits_clone = visits.clone();

                table_clone[i][j] = '#';

                if let Movement::Looped = simulate(&mut table_clone, &mut visits_clone)
                {
                    combos += 1;
                }

                println!("{}", count(&table_clone));
            }
        }
    }

    println!("{}", combos);
}