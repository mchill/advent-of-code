import * as fs from 'fs';

function getPermutations(array: string[], result: string[] = []): string[][] {
    if (array.length === 0) {
        return [result];
    }
    return [...Array(array.length).keys()].map((i) => {
        let cur = array.slice();
        let next = cur.splice(i, 1);
        return getPermutations(cur.slice(), result.concat(next));
    }).flat();
}

function part1() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    const outputs = lines.map((line) => line.split(' | ')[1].split(' '));
    return outputs.reduce((acc, output) => acc + output.filter((digit) => [2, 3, 4, 7].includes(digit.length)).length, 0);
}

function part2() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);

    const permutations = getPermutations('abcdefg'.split(''));
    let sum = 0;

    for (const line of lines) {
        const [patterns, output] = line.split(' | ').map((digits) => digits.split(' ').map((digit) => digit.split('').sort().join('')));

        for (const p of permutations) {
            const numbers = {
                [[p[0], p[1], p[2], p[4], p[5], p[6]].sort().join('')]: 0,
                [[p[2], p[5]].sort().join('')]: 1,
                [[p[0], p[2], p[3], p[4], p[6]].sort().join('')]: 2,
                [[p[0], p[2], p[3], p[5], p[6]].sort().join('')]: 3,
                [[p[1], p[2], p[3], p[5]].sort().join('')]: 4,
                [[p[0], p[1], p[3], p[5], p[6]].sort().join('')]: 5,
                [[p[0], p[1], p[3], p[4], p[5], p[6]].sort().join('')]: 6,
                [[p[0], p[2], p[5]].sort().join('')]: 7,
                [[p[0], p[1], p[2], p[3], p[4], p[5], p[6]].sort().join('')]: 8,
                [[p[0], p[1], p[2], p[3], p[5], p[6]].sort().join('')]: 9,
            };

            if (patterns.every((pattern) => Object.keys(numbers).includes(pattern))) {
                sum += Number(output.map((digit) => numbers[digit]).join(''));
                break;
            }
        }
    }

    return sum;
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
