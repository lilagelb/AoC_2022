use crate::day::{Answer, Day};

pub struct Day14;
impl Day for Day14 {
    type TypePart1 = u32;
    type TypePart2 = u32;

    fn run(&mut self) -> Answer<Self::TypePart1, Self::TypePart2> {
        let mut largest_x = 0;
        let mut largest_y = 0;

        let input: Vec<Vec<(usize, usize)>> = self.get_input_for_day_by_line(14).iter()
            .map(|line| line.split(" -> ")
                .map(|coords_str| {
                    let coords: Vec<usize> = coords_str.split(",")
                        .map(|ordinate| ordinate.parse().unwrap())
                        .collect();
                    if largest_x < coords[0] {
                        largest_x = coords[0];
                    }
                    if largest_y < coords[1] {
                        largest_y = coords[1];
                    }
                    (coords[0], coords[1])
                })
                .collect())
            .collect();

        let mut cave = Cave::new(largest_x, largest_y);
        cave.add_rocks(&input);

        let mut part_1 = 0;
        while cave.drop_sand((500, 0)).is_some() {
            part_1 += 1;
        }

        if largest_x < 500 + (largest_y + 2) {
            largest_x = 500 + (largest_y + 2);
        }
        let mut cave = Cave::new(largest_x, largest_y + 2);
        cave.add_rocks(&input);
        cave.add_line((0, largest_y + 2), (largest_x, largest_y + 2));

        let mut part_2 = 1;
        while cave.drop_sand((500, 0)).unwrap() != (500, 0) {
            part_2 += 1;
        }

        Answer::new(Some(part_1), Some(part_2))
    }
}
impl Day14 {
    pub fn new() -> Day14 {
        Day14
    }
}

#[derive(Clone)]
struct Cave {
    scan: Vec<Vec<CaveElement>>,
}
impl Cave {
    fn new(size_x: usize, size_y: usize) -> Cave {
        let mut scan = Vec::new();
        scan.resize_with(size_y+1, || {
            let mut line = Vec::new();
            line.resize(size_x+1, CaveElement::Air);
            line
        });
        Cave {
            scan,
        }
    }

    fn add_rocks(&mut self, rocks: &Vec<Vec<(usize, usize)>>) {
        for rock in rocks {
            for i in 0..(rock.len()-1) {
                self.add_line(rock[i], rock[i+1]);
            }
        }
    }

    fn add_line(&mut self, from: (usize, usize), to: (usize, usize)) {
        let mut xes = [from.0, to.0];
        let mut ys = [from.1, to.1];
        xes.sort();
        ys.sort();

        for x in xes[0]..=xes[1] {
            for y in ys[0]..=ys[1] {
                self.scan[y][x] = CaveElement::Rock;
            }
        }
    }

    fn drop_sand(&mut self, position: (usize, usize)) -> Option<(usize, usize)> {
        let mut x = position.0;
        let mut y = position.1;
        loop {
            let line = self.scan.get(y)?;
            match line[x] {
                CaveElement::Air => y += 1,
                CaveElement::Rock | CaveElement::Sand => {
                    if line.get(x-1)? == &CaveElement::Air {
                        x -= 1;
                    } else if line.get(x+1)? == &CaveElement::Air {
                        x += 1;
                    } else {
                        self.scan[y-1][x] = CaveElement::Sand;
                        return Some((x, y-1));
                    }
                },
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum CaveElement {
    Air,
    Rock,
    Sand,
}