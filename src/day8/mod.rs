use crate::day::{Answer, Day};

pub struct Day8 {
    grid: Vec<Vec<u32>>,
}
impl Day for Day8 {
    type TypePart1 = usize;
    type TypePart2 = usize;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        self.grid = self.get_input_for_day_by_line(8)
            .iter()
            .map(|line| line.as_bytes().iter().map(|elem| *elem as u32 - 48).collect())
            .collect();

        let mut part_1 = 2 * self.grid.len() + 2 * (self.grid[0].len() - 2);
        let mut part_2 = 0;

        for y in 1..self.grid.len()-1 {
            for x in 1..self.grid[0].len()-1 {
                let (visible, scenic_score)
                    = self.check_visibility_and_get_scenic_score(Vec2 { x, y });
                if visible {
                    part_1 += 1;
                }
                if scenic_score > part_2 {
                    part_2 = scenic_score;
                }
            }
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day8 {
    pub fn new() -> Day8 {
        Day8 { grid: Vec::new() }
    }

    fn check_visibility_and_get_scenic_score(&self, tree_coords: Vec2<usize>) -> (bool, usize) {
        let current_tree = self.grid[tree_coords.y][tree_coords.x];
        let mut directions_visible_from = 4;
        let mut total_scenic_score = 1;
        let mut direction_scenic_score = 0;

        for row in self.grid[..tree_coords.y].iter().rev() {
            direction_scenic_score += 1;
            if row[tree_coords.x] >= current_tree {
                directions_visible_from -= 1;
                break;
            }
        }
        total_scenic_score *= direction_scenic_score;
        direction_scenic_score = 0;
        for row in &self.grid[tree_coords.y + 1..] {
            direction_scenic_score += 1;
            if row[tree_coords.x] >= current_tree {
                directions_visible_from -= 1;
                break;
            }
        }
        total_scenic_score *= direction_scenic_score;
        direction_scenic_score = 0;
        for tree in self.grid[tree_coords.y][..tree_coords.x].iter().rev() {
            direction_scenic_score += 1;
            if *tree >= current_tree {
                directions_visible_from -= 1;
                break;
            }
        }
        total_scenic_score *= direction_scenic_score;
        direction_scenic_score = 0;
        for tree in &self.grid[tree_coords.y][tree_coords.x + 1..] {
            direction_scenic_score += 1;
            if *tree >= current_tree {
                directions_visible_from -= 1;
                break;
            }
        }
        total_scenic_score *= direction_scenic_score;
        (directions_visible_from > 0, total_scenic_score)
    }
}

struct Vec2<T> {
    x: T,
    y: T,
}