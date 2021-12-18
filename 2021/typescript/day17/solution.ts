import * as fs from 'fs';

function getBounds() {
    const input = fs.readFileSync(process.argv[2], 'utf-8').toString().trim();
    const [_, targetX0, targetX1, targetY0, targetY1] = input.match(/target area: x=(\d+)\.\.(\d+), y=(-\d+)\.\.(-\d+)/);
    return [targetX0, targetX1, targetY0, targetY1].map((number) => Number(number));
}

function part1(targetY0: number) {
    const yVelocity = Math.abs(targetY0) - 1;
    return yVelocity * (yVelocity + 1) / 2;
}

function part2(targetX0: number, targetX1: number, targetY0: number, targetY1: number) {
    let hits = 0;

    for (let xVelocity = 0; xVelocity <= targetX1; xVelocity++) {
        for (let yVelocity = targetY0; yVelocity <= Math.abs(targetY0); yVelocity++) {
            let x = 0;
            let y = 0;
            for (let step = 0; true; step++) {
                x += Math.max(xVelocity - step, 0);
                y += yVelocity - step;

                if (y >= targetY0 && y <= targetY1 && x >= targetX0 && x <= targetX1) {
                    hits++;
                    break;
                } else if (y < targetY0 || x > targetX1) {
                    break;
                }
            }
        }
    }

    return hits;
}

(() => {
    const [targetX0, targetX1, targetY0, targetY1] = getBounds();

    console.log('Part 1: ', part1(targetY0));
    console.log('Part 2: ', part2(targetX0, targetX1, targetY0, targetY1));
})();
