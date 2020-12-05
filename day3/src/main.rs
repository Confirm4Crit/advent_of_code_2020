fn main() {
    let input = include_str!("input.txt");
    let cols = input.find('\n').unwrap();
    let hill: Vec<usize> = input
        .lines()
        .map(|line| line.chars())
        .flatten()
        .map(|chr| if chr == '.' { 0 } else { 1 })
        .collect();
    let rows = hill.len() / cols;

    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes
        .iter()
        .map(|(sx, sy)| {
            let t = (0..rows)
                .step_by(*sy)
                .fold((0, 0), |(x, trees), y| {
                    (x + sx, trees + hill[y * cols + (x % cols)])
                })
                .1;
            println!("{},{} = {}", sx, sy, t);
            t
        })
        .product();

    println!("product: {}", product);
}
