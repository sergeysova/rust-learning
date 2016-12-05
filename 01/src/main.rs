
struct Usr {
    id: u32,
    health: u8,
}

trait Player {
    fn get_id(&self) -> u32;
    fn get_health(&self) -> u8;
    fn set_health(&mut self, new_health: u8);
}

impl Player for Usr {
    fn get_id(&self) -> u32 { self.id }

    fn get_health(&self) -> u8 { self.health }

    fn set_health(&mut self, new_health: u8)  {
        self.health = new_health;
    }
}

struct Admin;

impl Player for Admin {
    fn get_id(&self) -> u32 { 0 }

    fn get_health(&self) -> u8 { 100 }

    #[allow(unused_variables)]
    fn set_health(&mut self, new_health: u8)  {
    }
}

fn health_test<T: Player>(player: &mut T) {
    println!("Player {} has health {}", player.get_id(), player.get_health());
    println!("Patch it health with: 80");
    player.set_health(80);
    println!("Now player {} has health {}", player.get_id(), player.get_health());
}

fn main() {
    let mut user = Usr { id: 283745234, health: 100 };
    let mut admin = Admin;

    health_test(&mut user);
    health_test(&mut admin);

    println!("Test user: health is {}", user.health);
}
