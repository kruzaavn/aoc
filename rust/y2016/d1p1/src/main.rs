extern crate aoc;

#[tokio::main]
async fn main() {
    let data = match aoc::get_data(2016, 1).await{
        Ok(response) => response,
        Err(err) => {eprintln!("Error: {}", err); return;},
    };

    let mut pos: [i32; 2] = [0, 0];
    let mut facing: i32 = 0;

    for text in data.trim().split(", ") {

        let (turn, steps) = text.split_at(1);

        let num_steps = steps.parse::<i32>().unwrap();

        if turn == 'R'.to_string() {

            facing = (facing + 1) % 4

            } else {

            facing = ((facing - 1) + 4) % 4

            };

        if facing == 0 {

            pos[1] += num_steps;

        } else if facing == 2 {

            pos[1] -= num_steps;

        } else if facing == 1 {

            pos[0] += num_steps;

        } else {

            pos[0] -= num_steps;

        };


    }

    println!("{:?}", pos)
}
