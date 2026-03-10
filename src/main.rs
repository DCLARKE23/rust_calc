use std::io::{self};
use std::str::FromStr;

struct Operation {
    lhs: i32,
    rhs: i32,
    operator: Operator,
}

impl Operation {
    fn eval(&self) -> i32 {
        match self.operator {
            Operator::Add => {
                self.lhs + self.rhs
            },
            Operator::Subtract => {
                self.lhs - self.rhs
            },
            Operator::Multiply => {
                self.lhs * self.rhs
            },
            Operator::Divide => {
                if self.rhs == 0 {
                    panic!("Division by 0");
                }
                self.lhs / self.rhs
            },
            Operator::Exponent => {
                self.lhs.pow(self.rhs.try_into().unwrap())
            },
        }
    }
}

enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}


impl FromStr for Operator {
    type Err = String;

    fn from_str(input: &str) -> Result<Operator, Self::Err> {
        match input.trim() {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "^" => Ok(Operator::Exponent),
            _ => Err("Invalid Operator".to_string())
        }
    }
}

fn main() -> io::Result<()> {   
    let mut buf = String::with_capacity(5);

    println!("Enter the LHS");
    io::stdin().read_line(&mut buf)?;
    let lhs = buf.trim().parse().expect("Invalid input. Input must be an integer.");
    buf.clear();

    println!("Enter the RHS");
    io::stdin().read_line(&mut buf)?;
    let rhs = buf.trim().parse().expect("Invalid input. Input must be an integer.");
    buf.clear();

    println!("Enter the operator");
    io::stdin().read_line(&mut buf)?;
    let operator = buf.trim().parse().unwrap();
    buf.clear();
    
    let operation = Operation { lhs, rhs, operator };
    println!("{}", operation.eval());
    Ok(())
    
    
    // let mut left = String::new();
    // let mut right = String::new();
    // let mut op = String::new();

    // println!("Enter the LHS");
    // io::stdin().read_line(&mut left)?;
    // println!("Enter the RHS");
    // io::stdin().read_line(&mut right)?;
    // println!("Enter the operator");
    // io::stdin().read_line(&mut op)?;
    

    // let operation = Operation {
    //     lhs: left.trim().parse::<i32>().unwrap(),
    //     rhs: right.trim().parse::<i32>().unwrap(),
    //     operator: op.trim().parse().unwrap(),
    // };
    // println!("{:?}", operation.eval());
    // Ok(())
}