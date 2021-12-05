import * as fs from 'fs';

function solve() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().split(/\r?\n/).slice(0, -1);

    let position = 0;
    let depth = 0;
    let aim = 0;

    for (const line of lines) {
        const [direction, distanceStr] = line.split(' ');
        const distance = Number(distanceStr);

        switch (direction) {
            case 'forward':
                position += distance;
                depth += distance * aim;
                break;
            case 'down':
                aim += distance;
                break;
            case 'up':
                aim -= distance;
                break;
        }
    }

    return { position, depth, aim };
}

(() => {
    const { position, depth, aim } = solve();

    console.log('Part 1: ', position * aim);
    console.log('Part 2: ', position * depth);
})();
