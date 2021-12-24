import * as fs from 'fs';

function chunk(w: number, z: number, constants: number[]) {
    let x = z % 26 + constants[1];
    z = Math.floor(z / constants[0]);
    if (x !== w) {
        z = z * 26 + w + constants[2];
    }
    return z;
}

const cache = new Set<string>();

function recurse(z: number, constants: number[][], reverse: boolean, model: string): string {
    const key = `${model.length},${z}`;
    if (cache.has(key)) {
        return null;
    }
    cache.add(key);

    if (model.length === 14) {
        if (z === 0) {
            return model;
        }
        return null;
    }

    const ws = [...Array(9).keys()].map((w) => w + 1);
    if (reverse) {
        ws.reverse();
    }
    for (const w of ws) {
        const finalModel = recurse(chunk(w, z, constants[model.length]), constants, reverse, `${model}${w}`);
        if (finalModel != null) {
            return finalModel;
        }
    }

    return null;
}

function getConstants() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/).map((line) => line.split(' '));

    const constants: number[][] = [];
    for (let i = 0; i < lines.length; i += 18) {
        constants.push([Number(lines[i + 4][2]), Number(lines[i + 5][2]), Number(lines[i + 15][2])]);
    }

    return constants;
}

(() => {
    const constants = getConstants();

    console.log('Part 1: ', recurse(0, constants, true, ''));
    cache.clear();
    console.log('Part 2: ', recurse(0, constants, false, ''));
})();
