import * as fs from 'fs';
import * as _ from 'lodash';

class Scanner {
    public position = new Point([0, 0, 0]);
    public points: Point[];

    constructor(points: Point[]) {
        this.points = points;
    }

    private reorient(orientation: number) {
        for (const p of this.points) {
            p.reorient(orientation);
        };
    }

    private shift(offset: number[]) {
        this.position.shift(offset);
        for (const p of this.points) {
            p.shift(offset);
        }
    }

    public compare(other: Scanner) {
        for (let orientation = 0; orientation < 24; orientation++) {
            other.reorient(orientation);
            const offsets: { [key: string]: number } = {};

            for (const p1 of this.points) {
                for (const p2 of other.points) {
                    const offset = p1.diff(p2);
                    const key = offset.join(',');
                    offsets[key] = (offsets[key] || 0) + 1;

                    if (offsets[key] >= 12) {
                        other.shift(offset);
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

class Point {
    private _coords: number[];
    private orientation = 0;
    private offset = [0, 0, 0];

    constructor(point: number[]) {
        this.coords = point;
    }

    public get coords() {
        const direction = Math.floor(this.orientation / 4);
        const rotation = this.orientation % 4;

        const directionMap = {
            0: ([x, y, z]) => [x, y, z],
            1: ([x, y, z]) => [x, z, -y],
            2: ([x, y, z]) => [x, -z, y],
            3: ([x, y, z]) => [-z, y, x],
            4: ([x, y, z]) => [z, y, -x],
            5: ([x, y, z]) => [-x, y, -z],
        }
        const rotationMap = {
            0: ([x, y, z]) => [x, y, z],
            1: ([x, y, z]) => [-y, x, z],
            2: ([x, y, z]) => [-x, -y, z],
            3: ([x, y, z]) => [y, -x, z],
        }

        const position = rotationMap[rotation](directionMap[direction](this._coords));
        return [position[0] + this.offset[0], position[1] + this.offset[1], position[2] + this.offset[2]];
    }

    public set coords(coords: number[]) {
        this._coords = coords;
    }

    public reorient(orientation: number) {
        this.orientation = orientation;
    }

    public shift(offset: number[]) {
        this.offset = offset;
    }

    public diff(other: Point) {
        const coords = this.coords;
        const otherCoords = other.coords;
        return [coords[0] - otherCoords[0], coords[1] - otherCoords[1], coords[2] - otherCoords[2]];
    }

    public toString() {
        const coords = this.coords;
        return `${coords[0]},${coords[1]},${coords[2]}`;
    }
}

function adjustScanners() {
    const chunks = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n\r?\n/);
    const scanners = new Set(chunks.map((chunk) => new Scanner(chunk.split(/\r?\n/).slice(1).map((line) => new Point(line.split(',').map((number) => Number(number)))))));

    const [first] = scanners;
    scanners.delete(first);
    const locked = new Set([first]);

    while (scanners.size > 0) {
        for (const s1 of locked) {
            for (const s2 of scanners) {
                if (s1 !== s2 && s1.compare(s2)) {
                    scanners.delete(s2);
                    locked.add(s2);
                }
            }
        }
    }

    return locked;
}

function part1(scanners: Set<Scanner>) {
    return _.uniq([...scanners].map((scanner) => scanner.points).flat().map((p) => p.toString())).length;
}

function part2(scanners: Set<Scanner>) {
    let maxDistance = 0;

    for (const s1 of scanners) {
        for (const s2 of scanners) {
            maxDistance = Math.max(maxDistance, s1.position.diff(s2.position).reduce((a, b) => a + Math.abs(b), 0));
        }
    }

    return maxDistance;
}

(() => {
    const scanners = adjustScanners();

    console.log('Part 1: ', part1(scanners));
    console.log('Part 2: ', part2(scanners));
})();
