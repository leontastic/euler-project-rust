use super::Solve;

pub type Grid20x20 = [[usize; 20]; 20];

pub struct Parameters {
    pub input: Grid20x20,
}

type Line = [usize; 4];

fn horizontal4(g: Grid20x20, x: usize, y: usize) -> Line {
    [g[y][x], g[y][x + 1], g[y][x + 2], g[y][x + 3]]
}

fn vertical4(g: Grid20x20, x: usize, y: usize) -> Line {
    [g[y][x], g[y + 1][x], g[y + 2][x], g[y + 3][x]]
}

fn ascending4(g: Grid20x20, x: usize, y: usize) -> Line {
    [g[y + 3][x], g[y + 2][x + 1], g[y + 1][x + 2], g[y][x + 3]]
}

fn descending4(g: Grid20x20, x: usize, y: usize) -> Line {
    [g[y][x], g[y + 1][x + 1], g[y + 2][x + 2], g[y + 3][x + 3]]
}

fn product4(input: Line) -> usize {
    input.iter().product()
}

fn max_product4(g: Grid20x20, x: usize, y: usize, f: fn(Grid20x20, usize, usize) -> Line) -> usize {
    (0..y)
        .map(|y| (0..x).map(|x| f(g, x, y)).map(product4).max().unwrap())
        .max()
        .unwrap()
}

impl Solve for Parameters {
    fn solve(&self) -> Result<Option<String>, &str> {
        let Parameters { input } = *self;

        Ok(Some(format!(
            "{}",
            [
                max_product4(input, 17, 20, horizontal4),
                max_product4(input, 20, 17, vertical4),
                max_product4(input, 17, 17, ascending4),
                max_product4(input, 17, 17, descending4)
            ]
            .iter()
            .max()
            .unwrap()
        )))
    }
}
