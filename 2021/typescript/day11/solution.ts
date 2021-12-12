import * as fs from 'fs';

function getOctopuses() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);

    const octopuses = lines.map((line) => {
        const row = line.split('').map((char) => Number(char));
        row.unshift(-Infinity);
        row.push(-Infinity);
        return row;
    });

    octopuses.unshift(Array(octopuses[0].length).fill(-Infinity));
    octopuses.push(octopuses[0]);

    return octopuses;
}

function doStep(octopuses: number[][]) {
    octopuses.forEach((row, y) => row.forEach((energy, x) => octopuses[y][x] = energy + 1));

    while (octopuses.flat().filter((energy) => energy >= 10 && energy !== Infinity).length > 0) {
        octopuses.forEach((row, y) => row.forEach((energy, x) => {
            if (energy >= 10 && energy !== Infinity) {
                octopuses[y][x] = Infinity;
                octopuses[y][x + 1] += 1;
                octopuses[y + 1][x + 1] += 1;
                octopuses[y + 1][x] += 1;
                octopuses[y + 1][x - 1] += 1;
                octopuses[y][x - 1] += 1;
                octopuses[y - 1][x - 1] += 1;
                octopuses[y - 1][x] += 1;
                octopuses[y - 1][x + 1] += 1;
            }
        }));
    }

    const flashes = octopuses.flat().filter((energy) => energy === Infinity).length;
    octopuses.forEach((row, y) => row.forEach((energy, x) => octopuses[y][x] = energy === Infinity ? 0 : energy));
    return flashes;
}

function part1() {
    const octopuses = getOctopuses();
    let flashes = 0;

    for (let step = 0; step < 100; step++) {
        flashes += doStep(octopuses);
    }

    return flashes;
}

function part2() {
    const octopuses = getOctopuses();

    for (let step = 0; true; step++) {
        if (doStep(octopuses) === 100) {
            return step + 1;
        }
    }
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
