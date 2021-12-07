import * as fs from 'fs';

function solve(burn: (acc: number, crab: number) => number) {
    const crabs = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(',').map((crab) => Number(crab));
    const positions = Array.from({ length: Math.max(...crabs) - Math.min(...crabs) }, (_, i) => i + Math.min(...crabs));
    return Math.min(...positions.map((pos) => crabs.reduce((fuel, crab) => fuel + burn(pos, crab), 0)));
}

(() => {
    console.log('Part 1: ', solve((pos, crab) => Math.abs(crab - pos)));
    console.log('Part 2: ', solve((pos, crab) => (Math.pow(Math.abs(crab - pos), 2) + Math.abs(crab - pos)) / 2));
})();
