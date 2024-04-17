struct KeyPad {

    keys: [[i32; 3]; 3],
    position: [i32;2]

    }


impl KeyPad {

    fn get_number(&self, directions: &str) {


        }


    }



#[tokio::main]
async fn main() {
    let data = match aoc::get_data(2016, 2).await{
        Ok(response) => response,
        Err(err) => {eprintln!("Error: {}", err); return;},
    };

    let mut pad = KeyPad { keys:[[1,2,3],[4,5,6],[7,8,9]], position:[1,1]};

    let lines = data.lines().collect::<Vec<&str>>();

    println!{"{:?}", lines[0]}
}

