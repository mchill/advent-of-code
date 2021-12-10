import * as fs from 'fs';

function getHeightMap() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);

    const heightMap = lines.map((line) => `9${line}9`.split('').map((char) => Number(char)));
    heightMap.unshift(Array(heightMap[0].length).fill(9));
    heightMap.push(heightMap[0]);

    return heightMap;
}

function getLowPoints(heightMap: number[][]) {
    const lowPoints: number[][] = [];

    for (let row = 1; row < heightMap.length - 1; row++) {
        for (let col = 1; col < heightMap[row].length - 1; col++) {
            const cur = heightMap[row][col];
            if (cur < heightMap[row - 1][col] &&
                cur < heightMap[row + 1][col] &&
                cur < heightMap[row][col - 1] &&
                cur < heightMap[row][col + 1]) {
                lowPoints.push([row, col]);
            }
        }
    }
    
    return lowPoints;
}

function expandBasin(heightMap: number[][], basin: Set<string>, [row, col]: number[]) {
    return [[row - 1, col], [row + 1, col], [row, col - 1], [row, col + 1]].reduce((acc, [y, x]) => {
        if (!basin.has(`${y},${x}`) && heightMap[y][x] < 9) {
            basin.add(`${y},${x}`);
            acc += expandBasin(heightMap, basin, [y, x]) + 1;
        }
        return acc;
    }, 0);
}

function part1(heightMap: number[][], lowPoints: number[][]) {
    return lowPoints.reduce((acc, [row, col]) => acc + heightMap[row][col] + 1, 0);
}

function part2(heightMap: number[][], lowPoints: number[][]) {
    const basins = lowPoints.map(([row, col]) => expandBasin(heightMap, new Set<string>(`${row},${col}`), [row, col]));
    return basins.sort((a, b) => b - a).slice(0, 3).reduce((a, b) => a * b, 1);
}

(() => {
    const heightMap = getHeightMap();
    const lowPoints = getLowPoints(heightMap);

    console.log('Part 1: ', part1(heightMap, lowPoints));
    console.log('Part 2: ', part2(heightMap, lowPoints));
})();
