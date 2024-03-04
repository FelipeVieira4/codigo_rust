use std::io;

struct PlayerS{
    x: u16,
    y: u16,
    life: u8,
    is_dead: bool,
}

impl PlayerS{
    fn new(pos_x: u16,pos_y: u16) -> PlayerS{
        PlayerS{
        x:pos_x,
        y:pos_y,
        life:100,
        is_dead:false,
        }
    }
}

fn main() {
    
    let mut input = String::new();

    let test: i8;
    let player = PlayerS::new(30,60);
    
    io::stdin().read_line(&mut input).expect("Failed to read line");

    test = input.trim().parse().expect("Please enter a valid number");

    println!("x:{}, y:{}, life:{}, idDead:{}",player.x,player.y,player.life,player.is_dead);
    
    println!("Hello, world!");
    print!("{} :",test);
    
    println!("{}", match test < 100 {
    true => "is Less than 100",
    false => "is Bigger than or equal to 100"
    });

}