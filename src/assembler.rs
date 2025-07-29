use crate::instruction::{Hackable, Instruction};
use std::{collections::HashMap, fs};

#[allow(dead_code)]
pub struct Assembler {
    pub asm: Vec<String>,
    pub asm_without_symbols: Vec<String>,
    pub asm_without_labels: Vec<String>,
    pub asm_without_variables: Vec<String>,
    symbols: HashMap<String, String>,
}

impl Assembler {
    pub fn new(asm: Vec<String>) -> Self {
        Self {
            asm: asm,
            asm_without_symbols: vec![],
            asm_without_labels: vec![],
            asm_without_variables: vec![],
            symbols: HashMap::from([
                ("SP".to_string(), "0".to_string()),
                ("LCL".to_string(), "1".to_string()),
                ("ARG".to_string(), "2".to_string()),
                ("THIS".to_string(), "3".to_string()),
                ("THAT".to_string(), "4".to_string()),
                ("R0".to_string(), "0".to_string()),
                ("R1".to_string(), "1".to_string()),
                ("R2".to_string(), "2".to_string()),
                ("R3".to_string(), "3".to_string()),
                ("R4".to_string(), "4".to_string()),
                ("R5".to_string(), "5".to_string()),
                ("R6".to_string(), "6".to_string()),
                ("R7".to_string(), "7".to_string()),
                ("R8".to_string(), "8".to_string()),
                ("R9".to_string(), "9".to_string()),
                ("R10".to_string(), "10".to_string()),
                ("R11".to_string(), "11".to_string()),
                ("R12".to_string(), "12".to_string()),
                ("R13".to_string(), "13".to_string()),
                ("R14".to_string(), "14".to_string()),
                ("R15".to_string(), "15".to_string()),
                ("SCREEN".to_string(), "16384".to_string()),
                ("KBD".to_string(), "24576".to_string()),
            ]),
        }
    }

    pub fn from_file(program_name: &str) -> Result<Self, std::io::Error> {
        let asm = fs::read_to_string(program_name)?;

        let instructions = asm
            .lines()
            .map(|l| l.split("//").collect::<Vec<&str>>()[0])
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect::<Vec<String>>();

        Ok(Self::new(instructions))
    }

    pub fn compile(self: &Self) -> Result<Vec<String>, String> {
        let mut hack_out: Vec<String> = vec![];

        for op in &self.asm_without_variables {
            match Instruction::from(op.clone()) {
                Some(inst) => match inst.to_hack() {
                    Ok(inst_as_hack) => hack_out.push(inst_as_hack),
                    Err(err) => return Err(err),
                },
                None => return Err(format!("Could not parse {}", op)),
            }
        }

        Ok(hack_out)
    }

    pub fn pre_process(self: &mut Self) {
        self.replace_symbols();
        self.replace_labels();
        self.replace_variables();
    }

    // Step 1
    fn replace_symbols(self: &mut Self) {
        for op in self.asm.iter() {
            if !op.starts_with('@') {
                self.asm_without_symbols.push(op.clone());
                continue;
            }

            let (_, op_symbol) = op.split_at(1);
            match self.symbols.get(op_symbol) {
                Some(symbol) => self.asm_without_symbols.push(format!("@{symbol}")),
                None => self.asm_without_symbols.push(op.clone()),
            }
        }
    }

    // Step 2
    // TODO: labels have to be unique
    // TODO: non-existing labels cannot be referenced
    fn replace_labels(self: &mut Self) {
        let mut label_to_linenum: HashMap<String, String> = HashMap::new();

        let mut labels_count = 0;
        for (line_num, op) in self.asm_without_symbols.iter().enumerate() {
            if !op.starts_with('(') && !op.ends_with(')') {
                continue;
            }

            let label = op.trim_start_matches('(').trim_end_matches(')');
            label_to_linenum.insert(label.to_string(), (line_num - labels_count).to_string());
            labels_count += 1;
        }

        for op in self.asm_without_symbols.iter() {
            if op.starts_with('(') {
                continue;
            }

            if !op.starts_with('@') {
                self.asm_without_labels.push(op.clone());
                continue;
            }

            let (_, op_symbol) = op.split_at(1);
            match label_to_linenum.get(op_symbol) {
                Some(symbol) => self.asm_without_labels.push(format!("@{symbol}")),
                None => self.asm_without_labels.push(op.clone()),
            }
        }
    }

    // Step 3
    // TODO: can only allocate between @16 and @16383
    fn replace_variables(self: &mut Self) {
        let mut vars: Vec<(String, String)> = vec![];

        for op in self.asm_without_labels.iter() {
            if !op.starts_with('@') {
                self.asm_without_variables.push(op.clone());
                continue;
            }

            let (_, op_symbol) = op.split_at(1);
            if op_symbol.parse::<u16>().is_ok() {
                self.asm_without_variables.push(op.clone());
                continue;
            }

            if vars.is_empty() {
                vars.push((op_symbol.to_string(), "16".to_string()));
                self.asm_without_variables.push(format!("@16"));
                continue;
            }

            match vars.iter().find(|var| var.0 == op_symbol) {
                Some(v) => self.asm_without_variables.push(format!("@{}", v.1)),
                None => {
                    let next_addr = vars.last().unwrap().1.parse::<u16>().unwrap() + 1;
                    vars.push((op_symbol.to_string(), format!("{}", next_addr)));
                    self.asm_without_variables.push(format!("@{}", next_addr))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_symbols_are_replaced_correctly() {
        let mut assembler = Assembler::new(vec![
            "@R0".to_string(),
            "D=M".to_string(),
            "(TEST_LABEL)".to_string(),
        ]);

        assembler.replace_symbols();

        assert_eq!(
            vec![
                "@0".to_string(),
                "D=M".to_string(),
                "(TEST_LABEL)".to_string()
            ],
            assembler.asm_without_symbols
        );
    }

    #[test]
    fn labels_are_replaced_correctly() {
        let asm = vec![
            "@R0".to_string(),
            "D=M".to_string(),
            "@ZERO_OUTPUT".to_string(),
            "D;JEQ".to_string(),
            "(ZERO_OUTPUT)".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@R2".to_string(),
            "M=D".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
            "(INCR_D)".to_string(),
            "D=D+1".to_string(),
            "(END)".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
        ];

        let processed_asm = vec![
            "@0".to_string(),
            "D=M".to_string(),
            "@4".to_string(),
            "D;JEQ".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@2".to_string(),
            "M=D".to_string(),
            "@11".to_string(),
            "0;JMP".to_string(),
            "D=D+1".to_string(),
            "@11".to_string(),
            "0;JMP".to_string(),
        ];

        let mut assembler = Assembler::new(asm);
        assembler.replace_symbols();
        assembler.replace_labels();

        assert_eq!(processed_asm, assembler.asm_without_labels);
    }

    #[test]
    fn variables_are_replaced_correctly() {
        let asm = vec![
            "@R0".to_string(),
            "D=M".to_string(),
            "@a".to_string(),
            "M=M+1".to_string(),
            "@ZERO_OUTPUT".to_string(),
            "D;JEQ".to_string(),
            "(ZERO_OUTPUT)".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@R2".to_string(),
            "M=D".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
            "(INCR_D)".to_string(),
            "D=D+1".to_string(),
            "@b".to_string(),
            "M=M+1".to_string(),
            "(END)".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
        ];

        let processed_asm = vec![
            "@0".to_string(),
            "D=M".to_string(),
            "@16".to_string(),
            "M=M+1".to_string(),
            "@6".to_string(),
            "D;JEQ".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@2".to_string(),
            "M=D".to_string(),
            "@15".to_string(),
            "0;JMP".to_string(),
            "D=D+1".to_string(),
            "@17".to_string(),
            "M=M+1".to_string(),
            "@15".to_string(),
            "0;JMP".to_string(),
        ];

        let mut assembler = Assembler::new(asm);
        assembler.replace_symbols();
        assembler.replace_labels();
        assembler.replace_variables();

        assert_eq!(processed_asm, assembler.asm_without_variables);
    }

    #[test]
    fn compilation_works() {
        let asm = vec![
            "@R0".to_string(),
            "D=M".to_string(),
            "@a".to_string(),
            "M=M+1".to_string(),
            "@ZERO_OUTPUT".to_string(),
            "D;JEQ".to_string(),
            "(ZERO_OUTPUT)".to_string(),
            "@0".to_string(),
            "D=A".to_string(),
            "@R2".to_string(),
            "M=D".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
            "(INCR_D)".to_string(),
            "D=D+1".to_string(),
            "@b".to_string(),
            "M=M+1".to_string(),
            "(END)".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
        ];

        let expected: Vec<String> = vec![
            "0000000000000000".to_string(),
            "1111110000010000".to_string(),
            "0000000000010000".to_string(),
            "1111110111001000".to_string(),
            "0000000000000110".to_string(),
            "1110001100000010".to_string(),
            "0000000000000000".to_string(),
            "1110110000010000".to_string(),
            "0000000000000010".to_string(),
            "1110001100001000".to_string(),
            "0000000000001111".to_string(),
            "1110101010000111".to_string(),
            "1110011111010000".to_string(),
            "0000000000010001".to_string(),
            "1111110111001000".to_string(),
            "0000000000001111".to_string(),
            "1110101010000111".to_string(),
        ];

        let mut assembler = Assembler::new(asm);
        assembler.pre_process();

        assert_eq!(expected, assembler.compile().unwrap());
    }
}
