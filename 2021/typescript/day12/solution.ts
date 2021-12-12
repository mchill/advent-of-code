import * as fs from 'fs';

function getPaths() {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    const paths: { [key: string]: string[] } = {};

    for (const line of lines) {
        const [from, to] = line.split('-');
        paths[from] = paths[from] || [];
        paths[from].push(to);
        paths[to] = paths[to] || [];
        paths[to].push(from);
    }

    return paths;
}

function findPaths(paths: { [key: string]: string[] }, current: string, visited: Set<string>, spareTime: boolean) {
    if (current === 'end') {
        return 1;
    } else if (current === current.toLowerCase() && visited.has(current)) {
        if (current === 'start' || !spareTime) {
            return 0;
        }
        spareTime = false;
    }

    visited.add(current);
    return paths[current].reduce((acc, next) => acc + findPaths(paths, next, new Set(visited), spareTime), 0);
}

function part1(paths: { [key: string]: string[] }) {
    return findPaths(paths, 'start', new Set(), false);
}

function part2(paths: { [key: string]: string[] }) {
    return findPaths(paths, 'start', new Set(), true);
}

(() => {
    const paths = getPaths();

    console.log('Part 1: ', part1(paths));
    console.log('Part 2: ', part2(paths));
})();
