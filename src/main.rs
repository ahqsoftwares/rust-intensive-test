use rand::{thread_rng, Rng};

fn main() {
  let mut unique: Vec<u64> = Vec::new();
  let mut all: Vec<u64> = Vec::new();
  
  let mut index = 0;
  loop {
    let mut rng = thread_rng();
    let num: u64 = rng.gen();
    all.push(num);
    if index == i32::MAX {
      break;
    }
    index += 1;
  }
  
  for data in all.iter() {
    let repeats: Vec<&u64> = all.iter().filter(|x| {
      x == &data
    }).collect();

    println!("{}", &data);

    if repeats.len() == 1 {
      unique.push(data.clone());
    }
  }

  println!("Total unique: {}", unique.len());
}