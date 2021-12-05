import * as fs from 'fs';

function sum(numbers: number[]) {
    return numbers.reduce((a, b) => a + b, 0);
}

function convert(binary: number[]) {
    return parseInt(binary.join(''), 2);
}

function findRating(numbers: number[][], bitCriteria: (bits: number[]) => number) {
    let filteredNumbers = [...numbers];
    let currentBit = 0;
    while (filteredNumbers.length > 1) {
        const selectedBit = bitCriteria(filteredNumbers.map((num) => num[currentBit]));
        filteredNumbers = filteredNumbers.filter((num) => num[currentBit] === selectedBit);
        currentBit++;
    }
    return convert(filteredNumbers[0]);
}

function part1() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().split(/\r?\n/).slice(0, -1);
    const numbers = lines.map((line) => line.split('').map((char) => Number(char)));

    let bitSums = Array<number>(numbers[0].length).fill(0);
    bitSums = numbers.reduce((acc, num) => num.map((bit, i) => bit + acc[i]), bitSums);

    const gamma = convert(bitSums.map((bit) => +(bit > lines.length / 2)));
    const epsilon = convert(bitSums.map((bit) => +(bit < lines.length / 2)));

    return gamma * epsilon;
}

function part2() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().split(/\r?\n/).slice(0, -1);
    const numbers = lines.map((line) => line.split('').map((char) => Number(char)));

    const oxygen = findRating(numbers, (bits) => +(sum(bits) >= bits.length / 2));
    const c02 = findRating(numbers, (bits) => +(sum(bits) < bits.length / 2));

    return oxygen * c02;
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
