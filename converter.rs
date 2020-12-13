fn i8_to_u8(number: i8) -> u8 {
  let arr: [u8; 7] = [64, 32, 16, 8, 4, 2, 1];
  let mut calculator: i8 = 0;
  let mut result: u8 = 0;

  if (n < 0) {
    y += 128;
    x -= 128;
  }

  let mut index = 0;
  while (calculator != number) {
    if (calculator + arr[index] <= number) {
      calculator += arr[index];
      result += arr[index];
    }
    index += 1;
  }

  return result;
}