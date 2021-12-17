import * as fs from 'fs';

function parsePacket(binary: string, maxPackets: number = Infinity): [number, number, number[]] {
    let i = 0;
    let versions = 0;
    const literals: number[] = [];

    for (let packetCount = 0; i < binary.length && packetCount < maxPackets; packetCount++) {
        versions += parseInt(binary.slice(i, i+3), 2);
        const type = parseInt(binary.slice(i+3, i+6), 2);
        i += 6;

        if (type === 4) {
            let literal = '';
            do {
                literal += binary.slice(i+1, i+5);
                i += 5;
            } while (binary[i-5] !== '0');
            literals.push(parseInt(literal, 2));
            continue;
        }

        const lengthType = binary.slice(i, i+1);
        i++;

        let length: number;
        let subVersions: number;
        let subLiterals: number[];

        if (lengthType === '0') {
            length = parseInt(binary.slice(i, i+15), 2);
            i += 15;
            [length, subVersions, subLiterals] = parsePacket(binary.slice(i, i+length));
        } else if (lengthType === '1') {
            const count = parseInt(binary.slice(i, i+11), 2);
            i += 11;
            [length, subVersions, subLiterals] = parsePacket(binary.slice(i), count);
        }

        i += length;
        versions += subVersions;

        switch (type) {
            case 0:
                literals.push(subLiterals.reduce((a, b) => a + b, 0));
                break;
            case 1:
                literals.push(subLiterals.reduce((a, b) => a * b, 1));
                break;
            case 2:
                literals.push(Math.min(...subLiterals));
                break;
            case 3:
                literals.push(Math.max(...subLiterals));
                break;
            case 5:
                literals.push(+(subLiterals[0] > subLiterals[1]));
                break;
            case 6:
                literals.push(+(subLiterals[0] < subLiterals[1]));
                break;
            case 7:
                literals.push(+(subLiterals[0] === subLiterals[1]));
                break;
        }
    }

    return [i, versions, literals];
}

function solve() {
    const hex = fs.readFileSync(process.argv[2], 'utf-8').toString().trim();
    const binary = hex.split('').map((char) => ("0000" + parseInt(char, 16).toString(2)).slice(-4)).join('').replace(/0+$/, '');

    const [_, versions, literals] = parsePacket(binary);
    return [versions, literals];
}

(() => {
    const [versions, literals] = solve();

    console.log('Part 1: ', versions);
    console.log('Part 2: ', literals[0]);
})();
