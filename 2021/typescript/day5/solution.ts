import * as fs from 'fs';

function solve(excludeDiagonals: boolean = true) {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().split(/\r?\n/).slice(0, -1);
    const clouds = {};

    for (const line of lines) {
        const coords = line.split(' ');
        let [x1, y1] = coords[0].split(',').map((v) => Number(v));
        let [x2, y2] = coords[2].split(',').map((v) => Number(v));

        if (excludeDiagonals && x1 !== x2 && y1 !== y2) {
            continue;
        }

        const xStep = x2 > x1 ? 1 : x1 > x2 ? -1 : 0;
        const yStep = y2 > y1 ? 1 : y1 > y2 ? -1 : 0;

        for (let step = 0; step <= Math.max(Math.abs(x2 - x1), Math.abs(y2 - y1)); step++) {
            const current = `${x1 + xStep * step},${y1 + yStep * step}`;
            clouds[current] = (clouds[current] || 0) + 1;
        }
    }

    return Object.values(clouds).filter((v) => v > 1).length;
}

(() => {
    console.log('Part 1: ', solve());
    console.log('Part 2: ', solve(false));
})();
