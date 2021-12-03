import * as fs from 'fs';
import * as readline from 'readline';

async function solve() {
    const rl = readline.createInterface({
        input: fs.createReadStream(process.argv[2]),
    });

    let position = 0;
    let depth = 0;
    let aim = 0;

    for await (const line of rl) {
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

(async () => {
    const { position, depth, aim } = await solve();

    console.log('Part 1: ', position * aim);
    console.log('Part 2: ', position * depth);
})();
