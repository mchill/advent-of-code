import * as fs from 'fs';

function getInstructions(): [Set<string>, string[]] {
    const parts = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n\r?\n/);
    const [dots, folds] = parts.map((chunk) => chunk.split(/\r?\n/));

    let paper = new Set<string>();
    for (const line of dots) {
        const [x, y] = line.split(',');
        paper.add(`${x},${y}`);
    }

    return [paper, folds];
}

function doFold(paper: Set<string>, fold: string) {
    const foldUp = fold.includes('y=');
    const foldAt = Number(fold.split('=')[1]);

    const newPaper = new Set<string>();
    for (const dot of paper) {
        const [x, y] = dot.split(',').map((i) => Number(i));
        newPaper.add(foldUp ? `${x},${foldAt - Math.abs(y - foldAt)}` : `${foldAt - Math.abs(x - foldAt)},${y}`);
    }
    return newPaper;
}

function part1(paper: Set<string>, folds: string[]) {
    paper = doFold(paper, folds[0]);
    return paper.size;
}

function part2(paper: Set<string>, folds: string[]) {
    for (const fold of folds) {
        paper = doFold(paper, fold);
    }

    const output: string[][] = Array(6).fill(null).map(() => Array(40).fill(' '));
    for (const dot of paper) {
        const [x, y] = dot.split(',').map((i) => Number(i));
        output[y][x] = '#';
    }

    return output.map((row) => row.join('')).join('\n');
}

(() => {
    const [paper, folds] = getInstructions();

    console.log('Part 1: ', part1(paper, folds));
    console.log(`Part 2:\n${part2(paper, folds)}`);
})();
