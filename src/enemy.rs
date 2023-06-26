use crate::math;

pub struct EnemyManager{
    enemies: Vec<math::Pos2>,
    current_enemy_index: usize,
    enemy_symbol: char
}

impl EnemyManager{
    pub fn new(symbol: char, enemy_count: usize) -> Self{
        Self{
            enemies: Vec::with_capacity(enemy_count),
            current_enemy_index: 0,
            enemy_symbol: symbol,
        }
    }
    
    pub fn add_enemy(&mut self, pos: math::Pos2) -> usize{
        self.enemies.push(pos);
        let index = self.current_enemy_index;
        self.current_enemy_index += 1;
        index
    }
    
    pub fn update_enemies(&mut self, player_pos: &math::Pos2){
        for (index, enemy) in self.enemies.iter_mut().enumerate(){
            let pos = find_closest_position_to_player(enemy, player_pos);
            *enemy = *enemy + pos;
        }
    }
    
    pub fn get_enemy(&self, index: usize) -> &math::Pos2{
        &self.enemies[index]
    } 
    
    pub fn size(&self) -> usize{
        self.current_enemy_index
    }
}

fn find_closest_position_to_player(enemy: &math::Pos2, player: &math::Pos2) -> math::Pos2 {
    //println!("Player coordinates: {} | {} \n Enemy coordinates: {} | {}", player.x, player.y, enemy.x, enemy.y);
    let diff_x = (enemy.x - player.x).abs();
    let diff_y = (enemy.y - player.y).abs();
    //println!("Diff_x {} diff_y {}", diff_x, diff_y);

    //SPECIAL CASE WHEN ENEMY SPAWNS AT PLAYER POSITION
    if diff_x == 0 && diff_y == 0 {
        assert!(false, "Enemy spawned at player position");
        return math::Pos2::new(0, 0);
    }

    //move on the y_xis
    if diff_y > diff_x {
        if player.y > enemy.y {
            let offset = math::Pos2::new(0, 1);
            let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);

            if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
                assert!(false, "Enemy caught the player");
                math::Pos2::new(0, 0)
            } else {
                offset
            }
        } else {
            let offset = math::Pos2::new(0, -1);
            let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);

            if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
                assert!(false, "Enemy caught the player");
                math::Pos2::new(0, 0)
            } else {
                offset
            }
        }
    } else if player.x > enemy.x {
        let offset = math::Pos2::new(1, 0);
        let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);
        if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
            assert!(false, "Enemy caught the player");
            math::Pos2::new(0, 0)
        } else {
            offset
        }
    } else {
        let offset = math::Pos2::new(-1, 0);
        let pos_with_offset = math::Pos2::new(enemy.x + offset.x, enemy.y + offset.y);
        if player.x == pos_with_offset.x && player.y == pos_with_offset.y {
            assert!(false, "Enemy caught the player");
            math::Pos2::new(0, 0)
        } else {
            offset
        }
    }
}
