pub fn save_bool(b : bool) -> String {
  let a = if b {1} else {0};
  format!("{a}\n")
}

pub fn load_bool(s : String) -> bool {
  let a = s.trim();
  if a == "1" {
    true
  } else if a == "0" {
    false
  } else {
    panic!()
  }
}

pub fn save_u8(b : u8) -> String {
  format!("{b}\n")
}

pub fn load_u8(s : String) -> u8 {
  let a = s.trim();
  a.parse::<u8>().unwrap()
}

pub fn save_i32(d : i32) -> String {
  format!("{d}\n")
}

pub fn load_i32(s : String) -> i32 {
  let a = s.trim();
  a.parse::<i32>().unwrap()
}

pub fn save_u32 (d : u32) -> String {
  format!("{d}\n")
}

pub fn load_u32(s : String) -> u32 {
  let a = s.trim();
  a.parse::<u32>().unwrap()
}

pub fn save_option_u32 (d : Option<u32>) -> String {
  match d {
     Some(d) => format!("{d}\n"),
     None => "N\n".to_string(),
  }
}

pub fn load_option_u32(s : String) -> Option<u32> {
    let a = s.trim();
    if a == "N" {
      return None;
    } else {
      a.parse::<u32>().ok()
    }
  }


pub fn load_option_i32(s : String) -> Option<i32> {
  let a = s.trim();
  if a == "N" {
    return None;
  } else {
    a.parse::<i32>().ok()
  }
}

pub fn save_option_i32 (d : Option<i32>) -> String {
    match d {
       Some(d) => format!("{d}\n"),
       None => "N\n".to_string(),
    }
  }



pub fn save_string(s : String) -> String {
  format!("{}\n", s)
}

pub fn load_string(s : String) -> String {
  let a = s.trim();
  a.to_string()
}

pub fn save_vec_u32(v : Vec<u32>) -> String {
  let mut s = String::new();
  if v.len() == 0 {
    s.push_str("N\n");
  } else {
    s.push_str(&v.iter().map(|x| format!("{} ", x)).collect::<String>());
    s.push_str("\n");
  }
  s
}

pub fn load_vec_u32 (s : String) -> Vec<u32> {
  let mut v = Vec::new();
  let s = s.trim();
  if s == "N" {
    return v;
  } else {
    let mut s = s.split_whitespace();
    while let Some(s) = s.next() {
      v.push(s.parse::<u32>().unwrap());
    }
  }
  v
}
  