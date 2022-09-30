function getInput() {
    return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum State {
    Tree,
    Snow,
}

function main() {
    const result: State[][] = getInput()
        .split("\n")
        .map((x) =>
            x.split("").map((x) => (x === "." ? State.Snow : State.Tree))
        );

    const colLength = result[0].length;
    let treeeCount = 0;

    result.forEach((row, i) => {
        if (row[(i * 3) % colLength] === State.Tree) {
            treeeCount++;
        }
    });

    console.log("Hit trees", treeeCount);
}

main();
