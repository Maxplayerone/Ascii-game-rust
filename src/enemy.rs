use crate::math;

pub struct EnemyManager {
    enemies: Vec<math::Pos2>,
    current_enemy_index: usize,
}

impl EnemyManager {
    pub fn new(enemies: Vec<math::Pos2>) -> Self {
        let current_enemy_index = enemies.len();
        Self {
            enemies,
            current_enemy_index,
        }
    }

    pub fn update(&mut self, player_pos: &math::Pos2) {
        for enemy in self.enemies.iter_mut() {
            let pos = find_closest_position_to_player(enemy, player_pos);
            *enemy = *enemy + pos;
        }
    }

    pub fn get_enemy(&self, index: usize) -> &math::Pos2 {
        &self.enemies[index]
    }

    pub fn get_enemy_position(&self, index: usize) -> (usize, usize) {
        let enemy: &math::Pos2 = self.get_enemy(index);
        let x: usize = enemy.x.try_into().unwrap();
        let y: usize = enemy.y.try_into().unwrap();
        (x, y)
    }

    pub fn size(&self) -> usize {
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
