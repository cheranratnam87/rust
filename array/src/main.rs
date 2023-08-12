fn main() {
   let mut letters = ['a', 'b', 'c'];
   letters[0] = 'x';
   let first_letter = letters[0];
   println! ("first_letter is {}", first_letter);

   let numbers: [i32; 5];
   numbers = [0; 5];
   println!("last number is {}", numbers [4]);
}
