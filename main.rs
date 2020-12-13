static U16_MAX: u64 = u16::max_value() as u64;

fn main() {
  let mut stack = 0u64;
  
  println!("{}", stack_is_empty(&stack));
  add_to_stack(&mut stack, 34);
  add_to_stack(&mut stack, 0);
  println!("{}", stack_is_empty(&stack));
  println!("{}", peak_at_stack(&stack));
  println!("{}", stack_is_empty(&stack));
  println!("{}", pop_from_stack(&mut stack));
  println!("{}", peak_at_stack(&stack));
  println!("{}", stack_is_empty(&stack));
  println!("{}", pop_from_stack(&mut stack));
  println!("{}", peak_at_stack(&stack));
  println!("{}", stack_is_empty(&stack));
}

fn add_to_stack (stack: &mut u64, number: u16) {
  *stack = *stack * U16_MAX + number as u64;
}

fn peak_at_stack (stack: &u64) -> u16 {
  return (stack % U16_MAX) as u16;
}

fn pop_from_stack (stack: &mut u64) -> u16 {
  let value = peak_at_stack(stack);
  *stack = (*stack - value as u64) / U16_MAX;
  return value;
}

fn stack_is_empty (stack: &u64) -> bool {
  *stack == 0
}