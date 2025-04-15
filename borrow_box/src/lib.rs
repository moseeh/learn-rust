#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(self {
            id,
            p1_name, 0,
            p2_name, 0,
            nb_games,
        })
    }
    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 == self.p2.1 {
            return ("Same score! tied", self.p1(1));
        } else if self.p1.1 > self.p2.1 {
            return self.p1;
        }
        return self.p2;
    }
    pub fn update_score(&mut self, user_name: String) {
        let win = nb_games/2 +1 ;
        if self.p1.1 != win && self.p2.1 != win {

            if self.p1.0 == user_name {
                self.p1.1 += 1
            } else if self.p2.0 == user_name {
                self.p2.1 += 1
            }
        }
    }
    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}
