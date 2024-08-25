
use super::code_generator::{Generator, Instruction, Label};

use pest::Stack;
use strum_macros::Display;

#[derive(Display)]
pub enum INSTRUCTION { 
  MOV,
  INT,
  ADD,
  SUB,
  MUL, 
  DIV,
  PUSH,
  POP,
  CMP,
  JNZ,
  RET,
  CALL,
  INC,
  LOOP,
  DEC
}

pub fn gen_std_out_fn(gen : &mut Generator) { 

    gen.add_label(Label::from("print_fn"));
    
    // setup some registers we will need
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ebx", "10"])); // divisor
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ecx", "0"])); // character counter

    // Add newline char
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", "10"]));
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH,["dx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["ecx", "2"]));

    // covert base 2 to base 10 and push to stack
    gen.add_label(Label::from("convert_loop"));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", "0"]));
    gen.add_inst(Instruction::from(INSTRUCTION::DIV,["ebx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["edx", "'0'"]));
    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["ecx", "2"]));
    gen.add_inst(Instruction::from(INSTRUCTION::PUSH,["dx"]));
    // Are we done yet?
    gen.add_inst(Instruction::from(INSTRUCTION::CMP,["eax", "0"]));
    gen.add_inst(Instruction::from(INSTRUCTION::JNZ,["convert_loop"]));

    // std write the stack
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["edx", "ecx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ecx", "esp"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["ebx", "1"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["eax", "4"]));
    gen.add_inst(Instruction::from(INSTRUCTION::INT,["0x80"]));

    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["esp", "edx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::RET,[""]));

}

pub fn gen_animation(gen: &mut Generator, mut anim_stack: Stack<String>) {
 
  let loop_count = 10;
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["ecx", &loop_count.to_string()]));
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["ecx"]));
  gen.add_label(Label::from("anim_loop"));
 
  while let Some(anim_fn) =  anim_stack.pop() {
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["ecx", "[esp]"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["edx", &loop_count.to_string()]));
    gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["edx", "ecx"]));

    gen.add_inst(Instruction::from(INSTRUCTION::CALL, [anim_fn]));
  }
  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["ecx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::DEC, ["ecx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["ecx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::LOOP, ["anim_loop"]));
  
}