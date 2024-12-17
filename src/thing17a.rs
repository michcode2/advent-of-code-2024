struct Registers {
    A: usize,
    B: usize,
    C: usize,
}

#[derive(Clone, Copy, PartialEq)]
enum Opcode {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv
}

struct Device {
    registers: Registers,
    commands: Vec<Opcode>, 
    numbers: Vec<String>, 
    counter: usize,
    output: String,
}

impl Device {
    fn run(&mut self) {
        while self.counter < self.commands.len() {
            println!("{}, {}", self.counter, self.registers.B);
            let next_command = self.commands[self.counter];
            let next_operand= self.numbers[self.counter].clone();
            match next_command {
                Opcode::adv => self.adv(next_operand),
                Opcode::bxl => self.bxl(next_operand),
                Opcode::bst => self.bst(next_operand),
                Opcode::jnz => self.jnz(next_operand),
                Opcode::bxc => self.bxc(next_operand),
                Opcode::out => self.out(next_operand),
                Opcode::bdv => self.bdv(next_operand),
                Opcode::cdv => self.cdv(next_operand),
            }
            if next_command != Opcode::jnz{
                self.counter += 1;
            }
        }
        println!("{}", self.output);
    }

    fn adv(&mut self, operand: String) {
        let other = self.combo_operand(operand);
        let numerator = self.registers.A;
        let denominator = 2_usize.pow(other as u32);
        self.registers.A = numerator/denominator;
    }
    
    fn bxl(&mut self, operand: String) {
        let other = self.literal_operand(operand);
        self.registers.B = other ^ self.registers.B;
    }

    fn bst(&mut self, operand: String) {
        let other = self.combo_operand(operand);
        self.registers.B = other.rem_euclid(8);
    }

    fn jnz(&mut self, operand: String) {
        if self.registers.A == 0 {
            self.counter += 1;
            return;
        }
        let other = self.literal_operand(operand);
        self.counter = other / 2;
    }

    fn bxc(&mut self, operand: String) {
        self.registers.B = self.registers.B ^ self.registers.C;
    }

    fn out(&mut self, operand: String) {
        let other = self.combo_operand(operand);
        let value = other.rem_euclid(8);
        self.output = self.output.clone() + value.to_string().as_str();
        self.output = self.output.clone() + ",";
    }

    fn bdv(&mut self, operand: String) {
        let other = self.combo_operand(operand);
        let numerator = self.registers.A;
        let denominator = 2_usize.pow(other as u32);
        self.registers.B = numerator/denominator;
    }

    fn cdv(&mut self, operand: String) {
        let other = self.combo_operand(operand);
        let numerator = self.registers.A;
        let denominator = 2_usize.pow(other as u32);
        self.registers.C = numerator/denominator;
    }

    fn literal_operand(&self, operand: String) -> usize {
        operand.parse().unwrap()
    }

    fn combo_operand(&self, operand: String) -> usize {
        match operand.as_str() {
            "0" => 0,
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => self.registers.A,
            "5" => self.registers.B,
            "6" => self.registers.C,
            _ => panic!(),
        }
    }
}

fn csv_to_commands(csv: String) -> (Vec<Opcode>, Vec<String>) {
    let mut chunks = csv.split(",").collect::<Vec<&str>>();
    chunks.reverse();
    let mut opcodes = vec![];
    let mut operands = vec![];
    loop {
        if let Some(string) = chunks.pop() {
            match string {
                "0" => opcodes.push(Opcode::adv),
                "1" => opcodes.push(Opcode::bxl),
                "2" => opcodes.push(Opcode::bst),
                "3" => opcodes.push(Opcode::jnz),
                "4" => opcodes.push(Opcode::bxc),
                "5" => opcodes.push(Opcode::out),
                "6" => opcodes.push(Opcode::bdv),
                "7" => opcodes.push(Opcode::cdv),
                _ => panic!(),
            }
        } else {
            break;
        }
        operands.push(chunks.pop().unwrap().to_string());
    }

    (opcodes, operands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst(){
        let r = Registers{
            A: 0,
            B: 0,
            C: 9,
        };
        let opcodes = vec![Opcode::bst];
        let operands = vec!["6".to_string()];
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run();

        assert_eq!(computer.registers.B, 1);
    }
    
    #[test]
    fn test_two(){
        let r = Registers{
            A: 10,
            B: 0,
            C: 0,
        };
        let opcodes = vec![Opcode::out, Opcode::out, Opcode::out];
        let operands = vec!["0".to_string(), "1".to_string(), "4".to_string()];
        
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run();

        assert_eq!(computer.output, "0,1,2,");
    }

    #[test]
    fn test_three() {
        let r = Registers{
            A: 2024,
            B: 0,
            C: 0,
        };
        let (opcodes, operands) = csv_to_commands("0,1,5,4,3,0".to_string());
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run();
        assert_eq!(computer.output.as_str(), "4,2,5,6,7,7,7,7,3,1,0,");
        assert_eq!(computer.registers.A, 0);
    }

    #[test]
    fn test_four() {
        let r = Registers{
            A: 0,
            B: 29,
            C: 0,
        };
        let (opcodes, operands) = csv_to_commands("1,7".to_string());
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run();
        assert_eq!(computer.registers.B, 26);
    }

    #[test]
    fn test_five() {
        let r = Registers{
            A: 0,
            B: 2024,
            C: 43690,
        };
        let (opcodes, operands) = csv_to_commands("4,0".to_string());
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run();
        assert_eq!(computer.registers.B, 44354);
    }

    #[test]
    fn final_example() {
        let r = Registers{
            A: 729,
            B: 0,
            C: 0,
        };
        let (opcodes, operands) = csv_to_commands("0,1,5,4,3,0".to_string());
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run(); 
        assert_eq!(computer.output.as_str(), "4,6,3,5,6,3,5,2,1,0,");
    }

    #[test]
    fn actual_one() {
        let r = Registers{
            A: 64012472,
            B: 0,
            C: 0,
        };
        let (opcodes, operands) = csv_to_commands("2,4,1,7,7,5,0,3,1,7,4,1,5,5,3,0".to_string());
        let counter = 0;
        let mut computer = Device {
            registers: r,
            commands: opcodes,
            numbers: operands,
            counter,
            output: String::new(),
        };

        computer.run(); 
    }
}