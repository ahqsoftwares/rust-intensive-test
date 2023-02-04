use rand::{thread_rng, Rng};

fn main() {
  let mut unique: Vec<u64> = Vec::new();
  let mut all: Vec<u64> = Vec::new();
  
  let mut index: u128 = 0;
  loop {
    let mut rng = thread_rng();
    let num: u64 = rng.gen();
    all.push(num);
    if index == u128::MAX {
      break;
    }
    index += 1;
  }

  println!("Step II");
  
  for data in all.iter() {
    println!("{}", &data);
    let repeats: Vec<&u64> = all.iter().filter(|x| {
      x == &data
    }).collect();

    if repeats.len() == 1 {
      unique.push(data.clone());
    }
  }

  println!("Total unique: {}", unique.len());
}