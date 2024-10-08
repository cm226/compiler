use super::code_generator::{Generator, Instruction, Label};

use pest::Stack;
use strum_macros::Display;

#[allow(dead_code)]
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
  DEC,
  SYSCALL
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

    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rax", "1"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rdx", "rcx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rsi", "rsp"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV,["rdi", "1"]));
    gen.add_inst(Instruction::from(INSTRUCTION::SYSCALL, [""]));

    gen.add_inst(Instruction::from(INSTRUCTION::ADD,["rsp", "rdx"]));
    gen.add_inst(Instruction::from(INSTRUCTION::RET,[""]));

}

pub fn gen_animation(gen: &mut Generator, mut anim_stack: Stack<String>) {

  if anim_stack.len() == 0 {
    return;
  }
  let loop_count = 100;

  gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["create_window"]));
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rcx", &loop_count.to_string()]));
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rcx"]));
  gen.add_label(Label::from("anim_loop"));
 
  while let Some(anim_fn) =  anim_stack.pop() {
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rcx", "[rsp]"]));
    gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rdi", &loop_count.to_string()]));
    gen.add_inst(Instruction::from(INSTRUCTION::SUB, ["rdi", "rcx"]));

    gen.add_inst(Instruction::from(INSTRUCTION::CALL, [anim_fn]));
  }


  // TODO Why do i need to create a new stack frame here to call blit?
  // Am i doing something wrong, or is there a bug in blit code? 
  // AFAIK stack frame should be created gcc when generating the code for blit, so this stack frame 
  // should be redundant.... but if its not here i get seg fault..... confusing!!!
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rbp"]));
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rbp", "rsp"]));
  gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["blit"]));
  gen.add_inst(Instruction::from(INSTRUCTION::MOV, ["rsp", "rbp"]));
  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["rbp"]));

  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["rcx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::DEC, ["rcx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::PUSH, ["rcx"]));

  gen.add_inst(Instruction::from(INSTRUCTION::LOOP, ["anim_loop"]));
  
  gen.add_inst(Instruction::from(INSTRUCTION::POP, ["rcx"]));
  gen.add_inst(Instruction::from(INSTRUCTION::CALL, ["destroy_window"]));
  
}