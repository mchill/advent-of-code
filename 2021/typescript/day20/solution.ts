import * as fs from 'fs';
import * as _ from 'lodash';

function grow(image: string[], char: string) {
    image = image.map((line) => `${char}${char}${line}${char}${char}`);
    const line = new Array(image[0].length).fill(char).join('');
    image.unshift(line, line);
    image.push(line, line);
    return image;
}

function solve(enhancements: number) {
    const chunks = fs.readFileSync(process.argv[2], 'utf-8')
        .toString()
        .trim()
        .replaceAll('.', '0')
        .replaceAll('#', '1')
        .split(/\r?\n\r?\n/)
        .map((chunk) => chunk.split(/\r?\n/));

    const algorithm = chunks[0].join('');
    let image = chunks[1];

    for (let enhancement = 0; enhancement < enhancements; enhancement++) {
        const oldImage = grow(image, enhancement % 2 ? algorithm[0] : '0');
        image = [];

        for (let y = 1; y < oldImage.length - 1; y++) {
            image.push('');

            for (let x = 1; x < oldImage[y].length - 1; x++) {
                const binary = oldImage[y - 1].slice(x - 1, x + 2) + oldImage[y].slice(x - 1, x + 2) + oldImage[y + 1].slice(x - 1, x + 2);
                image[y - 1] += algorithm[parseInt(binary, 2)];
            }
        }
    }

    return _.countBy(image.map((line) => line.split('')).flat())['1'];
}

(() => {
    console.log('Part 1: ', solve(2));
    console.log('Part 2: ', solve(50));
})();
