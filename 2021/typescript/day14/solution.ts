import * as fs from 'fs';
import * as _ from 'lodash';

function solve(steps: number) {
    const parts = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n\r?\n/);
    const [part1, lines] = parts.map((chunk) => chunk.split(/\r?\n/));

    const polymer = part1[0];
    const rules: { [key: string]: string } = {};

    for (const line of lines) {
        const [match, insert] = line.split(' -> ');
        rules[match] = insert;
    }

    let pairs: { [key: string]: number } = {};
    const elements = _.countBy(polymer);

    for (let i = 0; i < polymer.length - 1; i++) {
        const pair = polymer.slice(i, i+2);
        pairs[pair] = (pairs[pair] || 0) + 1;
    }

    for (let step = 0; step < steps; step++) {
        const newPairs: { [key: string]: number } = {};

        for (const [pair, count] of Object.entries(pairs)) {
            const insert = rules[pair];
            const pair1 = `${pair[0]}${insert}`;
            const pair2 = `${insert}${pair[1]}`;

            newPairs[pair1] = (newPairs[pair1] || 0) + count;
            newPairs[pair2] = (newPairs[pair2] || 0) + count;

            elements[insert] = (elements[insert] || 0) + count;
        }

        pairs = newPairs;
    }

    return Math.max(...Object.values(elements)) - Math.min(...Object.values(elements));
}

(() => {
    console.log('Part 1: ', solve(10));
    console.log('Part 2: ', solve(40));
})();
