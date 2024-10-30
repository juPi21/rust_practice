fn gray(n: u8) -> Vec<String> {
  (0..2u32.pow(n.into()))
      .map(|i| {
          let gray_code = i ^ (i >> 1);
          format!("{:0width$b}", gray_code, width = n as usize)
      })
      .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
      let test_data = [
          (0, vec!("")),
          (1, vec!("0", "1")),
          (2, vec!("00", "01", "10", "11")),
          (3, vec!("000", "001", "010", "011", 
                   "100", "101", "110", "111")),
          (4, vec!("0000", "0001", "0010", "0011", 
                   "0100", "0101", "0110", "0111", 
                   "1000", "1001", "1010", "1011",
                   "1100", "1101", "1110", "1111")),
      ];

      test_data.iter().for_each(|(n, out)| 
          assert_eq!(gray(*n), *out)
      );
  }
}

fn main() {
  let result = gray(4);
  println!("Коди Грея для n = 4: {:?}", result);
  //Запустіть cargo test для перевірити тести
}
