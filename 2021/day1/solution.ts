import * as fs from 'fs';

function solve(windowSize: number) {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().split(/\r?\n/).slice(0, -1);

    let increaseCount = 0;
    let window = [];

    for (const line of lines) {
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

(() => {
    console.log('Part 1: ', solve(1));
    console.log('Part 2: ', solve(3));
})();
