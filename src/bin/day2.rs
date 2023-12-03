use std::io::Read;

fn main() {
    let mut content = String::new();
    let mut file = std::fs::File::open("input.txt").unwrap();

    file.read_to_string(&mut content).unwrap();
    let mut sum = 0;
    for line in content.lines() {
        let mut words = line.split(" ").collect::<Vec<&str>>();
        let id = std::str::from_utf8(&words[1].as_bytes()[..words[1].len() - 1])
            .unwrap()
            .parse::<i32>()
            .unwrap();

        words = words[2..].iter().map(|x| *x).collect::<Vec<&str>>();
        let game_str = words.join(" ");
        let game = game_str.split(";").collect::<Vec<&str>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in game {
            let a = round.trim().split(" ").collect::<Vec<&str>>();
            let mut i = 0;

            while i < a.len() {
                let num = a[i].parse::<i32>().unwrap();
                let color = a[i + 1].chars().filter(|x| *x != ',').collect::<String>();

                println!("Num = {num}, Color = {color}");
                match color.as_str() {
                    "red" => red = red.max(num),
                    "blue" => blue = blue.max(num),
                    "green" => green = green.max(num),
                    _ => (),
                }

                i += 2;
            }
        }

        sum += red * green * blue;
    }
    println!("{sum}");
}
