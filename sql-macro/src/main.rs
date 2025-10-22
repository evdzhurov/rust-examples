mod macros;

use sql_macro::Song;

fn main() {
    let db = vec![
        Song::new("Hate Me".to_string(), "Blue October".to_string(), 9),
        Song::new("Not Like Us".to_string(), "Kendrick Lamar".to_string(), 10),
        Song::new("Bad Dreams".to_string(), "Teddy Swims".to_string(), 10),
        Song::new(
            "Rockin' the Suburbs".to_string(),
            "Ben Folds".to_string(),
            6,
        ),
        Song::new("Lateralus".to_string(), "Tool".to_string(), 8),
        Song::new("Lose Control".to_string(), "Teddy Swims".to_string(), 9),
        Song::new("Come as you are".to_string(), "Nirvana".to_string(), 9),
    ];

    let results: Vec<(String,)> = query!(from db select title);
    // > [("Hate Me",), ("Not Like Us",), ("Bad Dreams",), ("Rockin' the Suburbs",), ("Lateralus",), ("Lose Control",), ("Come as you are",)]

    println!("{:?}", results);

    let results: Vec<(String, i64)> = query!(from db select title, rating);
    // > [("Hate Me", 9), ("Not Like Us", 10), ("Bad Dreams", 10), ("Rockin' the Suburbs", 6), ("Lateralus", 8), ("Lose Control", 9), ("Come as you are", 9)]

    println!("{:?}", results);

    let results: Vec<(String, i64)> =
        query!(from db select title, rating where rating == 10 and artist == "Teddy Swims");
    // > [("Bad Dreams", 10)]

    println!("{:?}", results);

    let results: Vec<(String, i64)> =
        query!(from db select title, rating where rating > 9 or artist == "Tool");

    println!("{:?}", results);
}
