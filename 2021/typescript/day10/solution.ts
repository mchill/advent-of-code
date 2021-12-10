import * as fs from 'fs';

function findValidLines() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);

    const pairMap = { '(': ')', '[': ']', '{': '}', '<': '>' };
    const valid: string[][] = [];
    const invalid: string[][] = [];

    for (const line of lines) {
        const stack: string[] = [];
        valid.push(stack);

        for (const char of line) {
            if (Object.keys(pairMap).includes(char)) {
                stack.push(char);
            } else if (char !== pairMap[stack.pop()]) {
                valid.pop();
                invalid.push([char]);
                break;
            }
        }
    }

    return [valid, invalid];
}

function part1(lines: string[][]) {
    const scoreMap = { ')': 3, ']': 57, '}': 1197, '>': 25137 };
    return lines.reduce((sum, line) => sum + scoreMap[line[0]], 0);
}

function part2(lines: string[][]) {
    const scoreMap = { '(': 1, '[': 2, '{': 3, '<': 4 };
    const scores = lines.map((line) => line.reverse().reduce((score, char) => score * 5 + scoreMap[char], 0));
    return scores.sort((a, b) => (a - b))[(scores.length - 1) / 2];
}

(() => {
    const [valid, invalid] = findValidLines();

    console.log('Part 1: ', part1(invalid));
    console.log('Part 2: ', part2(valid));
})();
