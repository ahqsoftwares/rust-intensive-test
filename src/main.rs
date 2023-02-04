use rand::{thread_rng, Rng};

fn main() {
  let mut unique: Vec<u64> = Vec::new();
  let mut all: Vec<u64> = Vec::new();

  println!("Generating Stuff");
  let mut rng = thread_rng();
  
  let mut index: u32 = 0;
  loop {
    let num: u64 = rng.gen();
    all.push(num);
    if index == u32::MAX {
      break;
    }
    index += 1;
  }

  println!("Step II");
  
  for data in all.clone().iter() {
    let cloned_all = all.clone();
    let repeats: Vec<&u64> = cloned_all.iter().filter(|x| {
      x == &data
    }).collect();

    std::thread::sleep_ms(2000);
    
    if repeats.len() == 1 {
      let new_all: Vec<u64> = all.clone().iter().filter(|x| {
        x == &data
      }).map(|value| {
        u64::from(value.clone().to_owned())
      }).collect();
      
      all = new_all.clone();
      unique.push(data.clone());
    }

    println!("Len: {}", all.len());
    
    drop(repeats);
    drop(cloned_all);
  }

  println!("Total unique: {}", unique.len());
}