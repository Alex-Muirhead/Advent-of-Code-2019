use std::fs::read_to_string;
use std::io::Error;

fn run(intcode: &mut Vec<usize>) -> usize {
    let len: usize = intcode.len();
    let mut pos: usize = 0;

    while pos < len-1 {
        let shift;
        let opcode = intcode[pos];

        match opcode {
            1  => shift = add(intcode, pos),
            2  => shift = mul(intcode, pos),
            99 => break,
            _  => panic!("Unknown opcode!")
        }

        pos += shift;
    }

    return intcode[0];
}

fn add(intcode: &mut Vec<usize>, pos: usize) -> usize {
    let memory_a = intcode[pos+1];
    let memory_b = intcode[pos+2];
    let output   = intcode[pos+3];

    intcode[output] = intcode[memory_a] + intcode[memory_b];
    return 4;
}

fn mul(intcode: &mut Vec<usize>, pos: usize) -> usize {
    let memory_a = intcode[pos+1];
    let memory_b = intcode[pos+2];
    let output   = intcode[pos+3];

    intcode[output] = intcode[memory_a] * intcode[memory_b];
    return 4;
}


fn main() -> Result<(), Error> {
    let path = "input.txt";
    let instructions = read_to_string(&path)?;

    let original_code: Vec<usize> = instructions
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect();

    'outer: for noun in 0..100 {
        'inner: for verb in 0..100 {
            // Reset memory to original values
            let mut intcode = original_code.clone();
            intcode[1] = noun;
            intcode[2] = verb;
            if run(&mut intcode) == 19690720 {
                println!("Required inputs are {}, {}", noun, verb);
                println!("Final output is {}", 100*noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}