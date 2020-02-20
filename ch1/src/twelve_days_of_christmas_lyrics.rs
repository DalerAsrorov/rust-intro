
pub fn start_program() {
  display_song_lyrics(12);
}

fn initialize_lyrics_lines() -> [String; 12] {
  let array: [String; 12] = [
    "A partridge in a pear tree".to_string(),
    "turtle doves".to_string(),
    "french hens".to_string(),
    "calling birds".to_string(),
    "golden rings".to_string(),
    "geese a-laying".to_string(),
    "swans a-swimming".to_string(),
    "maids a-milking".to_string(),
    "ladies dancing".to_string(),
    "lords a-leaping".to_string(),
    "pipers piping".to_string(),
    "drummers drumming".to_string()
  ];

  return array;
}

fn starting_lyric(num: i32) -> String {
  return ["On the", &num.to_string(), "day of Christmas my true love sent to me"].join(" ")
}

fn display_song_lyrics(num: i32) {
  let lyrics_array: [String; 12] = initialize_lyrics_lines();

  for i in 1..num + 1 {
    let start = starting_lyric(i);
    println!("\t{}", start);

    for j in (0..i).rev() {
      match j {
        0 => println!("\t{}", lyrics_array[j as usize]),
        _ => {
          let add_and = match j {
            1 => ", and".to_string(),
            _ => "".to_string()
          };

          println!("\t{} {}{}", j + 1, lyrics_array[j as usize], add_and);
        }
      }
    }

    println!("\n");
  }
}