import * as fs from 'fs';

function solve(days: number) {
    let fishes = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(',').map((fish) => Number(fish));
    fishes = [...Array(9).keys()].map((i) => fishes.filter((fish) => fish === i).length);

    for (let day = 0; day < days; day++) {
        fishes.push(fishes.shift());
        fishes[6] += fishes[8];
    }

    return fishes.reduce((a, b) => a + b, 0);
}

(() => {
    console.log('Part 1: ', solve(80));
    console.log('Part 2: ', solve(256));
})();
