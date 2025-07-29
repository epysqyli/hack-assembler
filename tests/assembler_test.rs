use hack_assembler::assembler;

#[test]
fn assembler_is_initialized_from_file() {
    let assembler = assembler::Assembler::from_file("tests/fixtures/mult.asm");
    assert!(assembler.is_ok())
}

#[test]
fn asm_file_is_parsed_correctly() {
    let expected = vec![
        "@R0",
        "D=M",
        "@ZERO_OUTPUT",
        "D;JEQ",
        "@R1",
        "D=M",
        "@ZERO_OUTPUT",
        "D;JEQ",
        "@R2",
        "M=0",
        "@R1",
        "D=M",
        "@i",
        "M=D",
        "(LOOP)",
        "@i",
        "D=M",
        "@END",
        "D;JEQ",
        "@R2",
        "D=M",
        "@R0",
        "D=D+M",
        "@R2",
        "M=D",
        "@i",
        "M=M-1",
        "@LOOP",
        "0;JMP",
        "(ZERO_OUTPUT)",
        "@0",
        "D=A",
        "@R2",
        "M=D",
        "@END",
        "0;JMP",
        "(END)",
        "@END",
        "0;JMP",
    ];

    assembler::Assembler::from_file("tests/fixtures/mult.asm")
        .unwrap()
        .asm
        .iter()
        .enumerate()
        .for_each(|(line_num, instr)| assert_eq!(expected[line_num], instr));
}
