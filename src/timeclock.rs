// fn foo_implementation() {
//     println!("Hello, world!");
// }

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests;
// mod tests {
//   // fn test_foo_implementation() {}
    
//   use super::*;

//   #[test]
//   fn it_works() {
//       let result = add(2, 2);
//       assert_eq!(result, 4);
//   }

// }
