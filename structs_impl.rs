
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
    
    let player = PlayerS::new(30,60);
    

    println!("x:{}, y:{}, life:{}, idDead:{}",player.x,player.y,player.life,player.is_dead);
}