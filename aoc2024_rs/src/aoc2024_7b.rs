use std::{fmt::Error, str::FromStr};

struct Equation
{
    lhs: u64,
    rhs: Vec<u64>,
}

impl FromStr for Equation
{
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(':')
        {
            if let Ok(lhs) = a.parse::<u64>()
            {
                let rhs: Vec<u64> = b.split_ascii_whitespace().filter_map(|t| t.parse::<u64>().ok()).collect();
                return Ok(Equation { lhs: lhs, rhs: rhs});
            }
        }

        return Err(Error);
    }
}

impl Equation
{
    pub fn is_satisfiable(&self) -> bool
    {
        self.is_satisfiable_rec(0, 0)
    }

    fn concat(lhs: u64, rhs: u64) -> u64
    {
        lhs * 10u64.pow(1u32 + rhs.checked_ilog10().unwrap()) + rhs
    }

    fn is_satisfiable_rec(&self, offset: usize, cur_value: u64) -> bool
    {
        if offset == self.rhs.len()
        {
            cur_value == self.lhs
        }
        else
        {
            let x = self.rhs[offset];
            self.is_satisfiable_rec(offset + 1, cur_value + x) ||
                self.is_satisfiable_rec(offset + 1, cur_value * x) ||
                self.is_satisfiable_rec(offset + 1, Self::concat(cur_value, x))
        }
    }
}

pub fn main(input: Vec<String>)
{
    let equations: Vec<Equation> = input.iter().filter_map(|line| line.parse::<Equation>().ok()).collect();
    let satisfiable = equations.iter().filter(|eq| eq.is_satisfiable());
    let sum: u64 = satisfiable.map(|eq| eq.lhs).sum();
    println!("{}", sum);
}