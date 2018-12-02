import * as fs from 'fs';
import * as readline from 'readline';

async function solve(windowSize: number) {
    const rl = readline.createInterface({
        input: fs.createReadStream(process.argv[2]),
    });

    let increaseCount = 0;
    let window = [];

    for await (const line of rl) {
        const depth = Number(line);
        window.push(depth);

        if (window.length <= windowSize) {
            continue;
        }

        const oldDepthSum = window.reduce((a, b) => a + b, 0) - depth;
        window.shift();
        const newDepthSum = window.reduce((a, b) => a + b, 0);

        increaseCount += +(newDepthSum > oldDepthSum);
    }

    return increaseCount;
}

(async () => {
    console.log('Part 1: ', await solve(1));
    console.log('Part 2: ', await solve(3));
})();
