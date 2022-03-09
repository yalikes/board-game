use rand;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct GameModel {
    pub gamemap: GameMap,
    pub select_1: Option<(usize, usize)>,
    pub select_2: Option<(usize, usize)>,
}

impl GameModel {
    pub fn new() -> Self {
        let gamemap = GameMap::new();
        GameModel {
            gamemap,
            select_1: None,
            select_2: None,
        }
    }
}

#[derive(Debug)]
pub struct GameMap {
    pub map: Vec<Vec<Option<i32>>>,
}

impl GameMap {
    pub fn new() -> Self {
        let mut s = generate_shuffled_list(8);
        if reverse_number(&s) % 2 == 1 {
            let temp = s[0];
            s[0] = s[1];
            s[1] = temp;
        } //make list reverse order number to be even
        let mut map: Vec<Vec<Option<i32>>> = Vec::new();
        for i in 0..3 {
            map.push(Vec::new());
            for j in 0..3 {
                if i == 2 && j == 2 {
                    map[i].push(None)
                } else {
                    map[i].push(Some(s[i * 3 + j] as i32));
                }
            }
        }
        GameMap { map }
    }

    pub fn swap_pos(&mut self, pos1: (usize, usize), pos2: (usize, usize)) {
        let temp = self.map[pos1.0][pos1.1];
        self.map[pos1.0][pos1.1] = self.map[pos2.0][pos2.1];
        self.map[pos2.0][pos2.1] = temp;
    }

    pub fn is_valid_action(&self, pos1: (usize, usize), pos2: (usize, usize)) -> bool {
        if pos1 == pos2 {
            return false;
        } else if self.map[pos1.0][pos1.1].is_none() || self.map[pos2.0][pos2.1].is_none() {
            return (pos1.0 as i32 - pos2.0 as i32).abs() + (pos1.1 as i32 - pos2.1 as i32).abs()
                == 1;
        }
        return false;
    }

    pub fn is_valid_selection(&self, pos: (usize, usize)) -> bool {
        let mut none_pos = (0, 0);
        for i in 0..3 {
            for j in 0..3 {
                if self.map[i][j].is_none() {
                    none_pos.0 = i;
                    none_pos.1 = j;
                }
            }
        }
        pos == none_pos
            || ((pos.0 as i32 - none_pos.0 as i32).abs() + (pos.1 as i32 - none_pos.1 as i32).abs())
                == 1
    }

    pub fn is_completed(&self) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if i == 2 && j == 2 {
                    if self.map[i][j] != None {
                        return false;
                    }
                } else if self.map[i][j] != Some((3 * i + j + 1) as i32) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn generate_shuffled_list(nsize: u32) -> Vec<u32> {
    let mut list: Vec<u32> = (1..=nsize).collect();
    let mut rng = rand::thread_rng();
    list.as_mut_slice().shuffle(&mut rng);
    list
}

pub fn reverse_number<T: Ord>(list: &Vec<T>) -> u32 {
    let n = list.len();
    let mut reverse_counter = 0;
    for i in 0..n {
        for j in 0..i {
            if list[j] > list[i] {
                reverse_counter += 1;
            }
        }
    }
    reverse_counter
}

#[test]
fn test_reverse_number() {
    assert_eq!(reverse_number(&vec![1, 2, 3]), 0);
    assert_eq!(reverse_number(&vec![4, 3, 1, 2]), 5);
}
