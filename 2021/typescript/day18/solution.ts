import * as fs from 'fs';

type SnailfishNumber = Array<SnailfishNumber | number>;

function add(numbers: string[]) {
    let number = numbers[0];

    for (let n = 1; n < numbers.length; n++) {
        number = `[${number},${numbers[n]}]`;

        while (true) {
            let exploded = false;
            let split = false;

            let depth = 0;

            for (let i = 0; i < number.length; i++) {
                depth += +(number[i] === '[') - +(number[i] === ']');
                if (depth < 5) {
                    continue;
                }

                const [match, left, right] = number.slice(i).match(/\[(\d+),(\d+)\]/);
                const toLeft = number.slice(0, i).replace(/^(.*[^\d])(\d+)([^\d].*)$/, (_, p1, p2, p3) => `${p1}${Number(p2) + Number(left)}${p3}`);
                const toRight = number.slice(i + match.length).replace(/^([^\d]+)(\d+)([^\d].*)$/, (_, p1, p2, p3) => `${p1}${Number(p2) + Number(right)}${p3}`);
                number = `${toLeft}0${toRight}`;

                exploded = true;
                break;
            }

            if (exploded) {
                continue;
            }

            number = number.replace(/^(.*?[^\d])(\d{2,})([^\d].*)$/, (_, p1, p2, p3) => {
                split = true;
                return `${p1}[${Math.floor(Number(p2) / 2)},${Math.ceil(Number(p2) / 2)}]${p3}`;
            });

            if (!split) {
                break;
            }
        }
    }

    return getMagnitude(JSON.parse(number));
}

function getMagnitude(number: SnailfishNumber | number) {
    if (typeof number === 'number') {
        return number;
    }
    return 3 * getMagnitude(number[0]) + 2 * getMagnitude(number[1]);
}

function part1() {
    const numbers = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    return add(numbers);
}

function part2() {
    const numbers = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    let max = 0;

    for (let first = 0; first < numbers.length; first++) {
        for (let second = 0; second < numbers.length; second++) {
            if (first === second) {
                continue;
            }
            max = Math.max(max, add([numbers[first], numbers[second]]))
        }
    }

    return max;
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
