import * as fs from 'fs';
import * as _ from 'lodash';

function part1() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    let cucumbers = lines.map((line) => line.split(''));

    let step = 0;
    let moved: number;

    for (; moved !== 0; step++) {
        moved = 0;

        let newCucumbers = _.cloneDeep(cucumbers);
        for (let y = 0; y < cucumbers.length; y++) {
            for (let x = 0; x < cucumbers[y].length; x++) {
                const nextX = (x + 1) % cucumbers[y].length;
                if (cucumbers[y][x] === '>' && cucumbers[y][nextX] === '.') {
                    newCucumbers[y][x] = '.';
                    newCucumbers[y][nextX] = '>';
                    moved++;
                }
            }
        }
        cucumbers = newCucumbers;

        newCucumbers = _.cloneDeep(cucumbers);
        for (let y = 0; y < cucumbers.length; y++) {
            for (let x = 0; x < cucumbers[y].length; x++) {
                const nextY = (y + 1) % cucumbers.length;
                if (cucumbers[y][x] === 'v' && cucumbers[nextY][x] === '.') {
                    newCucumbers[y][x] = '.';
                    newCucumbers[nextY][x] = 'v';
                    moved++;
                }
            }
        }
        cucumbers = newCucumbers;
    }

    return step;
}

(() => {
    console.log('Part 1: ', part1());
})();
