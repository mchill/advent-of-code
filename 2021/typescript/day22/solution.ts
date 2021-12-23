import * as fs from 'fs';

type Cuboid = { x: number[], y: number[], z: number[] };

function part1() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    const cubes = new Set<string>();

    for (const line of lines) {
        const [instruction, dimensions] = line.split(' ');
        const [xRange, yRange, zRange] = dimensions.split(',').map((dimension) => dimension.substring(2).split('..').map((number) => Number(number)));

        for (let x = Math.max(xRange[0], -50); x <= Math.min(xRange[1], 50); x++) {
            for (let y = Math.max(yRange[0], -50); y <= Math.min(yRange[1], 50); y++) {
                for (let z = Math.max(zRange[0], -50); z <= Math.min(zRange[1], 50); z++) {
                    if (instruction === 'on') {
                        cubes.add(`${x},${y},${z}`);
                    } else {
                        cubes.delete(`${x},${y},${z}`);
                    }
                }
            }

        }
    }

    return cubes.size;
}

function removeOverlapping(c1: Cuboid, c2: Cuboid): Cuboid[] {
    if (c1.x[0] > c2.x[1] || c1.x[1] < c2.x[0] ||
        c1.y[0] > c2.y[1] || c1.y[1] < c2.y[0] ||
        c1.z[0] > c2.z[1] || c1.z[1] < c2.z[0]) {
        return [c2];
    }

    const cuboids: Cuboid[] = [];

    if (c1.z[0] > c2.z[0]) {
        cuboids.push({ x: c2.x, y: c2.y, z: [c2.z[0], c1.z[0] - 1] });
    }
    if (c1.z[1] < c2.z[1]) {
        cuboids.push({ x: c2.x, y: c2.y, z: [c1.z[1] + 1, c2.z[1]] });
    }
    if (c1.y[0] > c2.y[0]) {
        cuboids.push({ x: c2.x, y: [c2.y[0], c1.y[0] - 1], z: [Math.max(c1.z[0], c2.z[0]), Math.min(c1.z[1], c2.z[1])] });
    }
    if (c1.y[1] < c2.y[1]) {
        cuboids.push({ x: c2.x, y: [c1.y[1] + 1, c2.y[1]], z: [Math.max(c1.z[0], c2.z[0]), Math.min(c1.z[1], c2.z[1])] });
    }
    if (c1.x[0] > c2.x[0]) {
        cuboids.push({ x: [c2.x[0], c1.x[0] - 1], y: [Math.max(c1.y[0], c2.y[0]), Math.min(c1.y[1], c2.y[1])], z: [Math.max(c1.z[0], c2.z[0]), Math.min(c1.z[1], c2.z[1])] });
    }
    if (c1.x[1] < c2.x[1]) {
        cuboids.push({ x: [c1.x[1] + 1, c2.x[1]], y: [Math.max(c1.y[0], c2.y[0]), Math.min(c1.y[1], c2.y[1])], z: [Math.max(c1.z[0], c2.z[0]), Math.min(c1.z[1], c2.z[1])] });
    }

    return cuboids;
}

function part2() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);

    const cuboids: Cuboid[] = [];

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        const [instruction, dimensions] = line.split(' ');
        const [x, y, z] = dimensions.split(',').map((dimension) => dimension.substring(2).split('..').map((number) => Number(number)));

        for (let c = 0; c < cuboids.length;) {
            const splitCuboids = removeOverlapping({ x, y, z }, cuboids[c]);
            cuboids.splice(c, 1, ...splitCuboids)
            c += splitCuboids.length;
        }

        if (instruction === 'on') {
            cuboids.push({ x, y, z });
        }
    }

    return cuboids.reduce((acc, cuboid) => acc + Object.values(cuboid).reduce((acc, range) => acc * (range[1] - range[0] + 1), 1), 0);
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
