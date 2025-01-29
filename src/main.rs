use std::io;

fn main() {
    println!("Enter your skin tone(warm, cool, neutral): ");
    let mut skin_tone = String::new();
    io::stdin().read_line(&mut skin_tone).expect("Failed to read line");
    let skin_tone = skin_tone.trim();

    // Define color and jewelry recommendations for each skin tone
    let (colors, jewelry) = match skin_tone.to_lowercase().as_str() {
        "warm" => (
            vec![
                "brown","beige", "camel", "terracotta", "tomato red", "coral", "mustard", "gold",
                "burnt orange", "olive green", "sage", "forest green", "plum"
            ],
	    vec!["gold jewelry", "copper jewelry", "earth-toned stones"]
        ),
        "cool" => (
	    vec![
	        "navy blue", "royal blue", "ice blue", "lavender", "violet", "fuchsia", "teal",
	        "mint green", "emerald green", "cherry kid", "pure white", "pearl white", "silver"
            ],
	    vec!["silver jewelry", "platinum jewelry", "gemstones like sapphire and emerald"]
        ),
        "neutral" => (
	    vec![
	        "taupe", "charcoal grey", "white", "black", "ivory", "blush pink", "mint", "lavender",
	        "jade green", "turquoise", "berry red", "navy blue", "soft teal"
	    ],
	    vec!["rose gold jewelry", "white gold jewelry", "diamond jewelry "]
        ),
        _ => (
            vec!["Invalid input. Please enter warm,cool, or neutral."],
            vec![]
        ), 
    
     };

   // Display color recommendations
   println!("Recommend colors for {} skin tone: {:?}", skin_tone, colors);

  //Display jewelry recommendations if the list is not empty
  if !jewelry.is_empty() {
      println!("Recommend jewelry for {} skin tone: {:?}", skin_tone, jewelry);
  } else {
      println!("No jewelry recommnendations available.");
  }  
}
