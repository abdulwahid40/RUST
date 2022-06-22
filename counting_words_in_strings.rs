// Counting words in string

use std::{collections::HashMap, hash::Hash};

fn main() {
 
   let text = "hello world wonderful world";
   
   let mut map = HashMap::new();

   // ["hello", "world", "wonderful", "world"]
   for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
   }

   println!("{:?}", map);

}
