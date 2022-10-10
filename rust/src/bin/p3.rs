fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let result = get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| return line.chars().nth(idx * 3 % line.len()))
        .filter(|&x| x == '#')
        .count();

    println!("count trees {}", result);
}
