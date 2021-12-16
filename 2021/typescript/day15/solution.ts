import * as fs from 'fs';

class Node {
    public edges: Node[] = [];
    public csf = 0;
    public cost: number;

    public constructor(cost: number) {
        this.cost = cost;
    }
}

function findShortestPath(grid: Node[][]) {
    for (let y = 0; y < grid.length; y++) {
        for (let x = 0; x < grid[y].length; x++) {
            let surrounding = [[x+1, y], [x, y+1], [x-1, y], [x, y-1]];
            surrounding = surrounding.filter(([col, row]) => col >= 0 && col < grid[y].length && row >= 0 && row < grid.length);
            grid[y][x].edges = surrounding.map(([col, row]) => grid[row][col]);
        }
    }

    const nodes = grid.flat();
    const target = nodes[nodes.length - 1];

    const openList = new Set([nodes[0]]);
    const closedList = new Set<Node>();

    while (openList.size > 0) {
        const node = [...openList].sort((a, b) => a.csf - b.csf)[0];

        for (const next of node.edges) {
            const csf = node.csf + next.cost + 1;

            if (openList.has(next)) {
                if (csf < next.csf) {
                    next.csf = csf;
                }
            } else if (!closedList.has(next)) {
                next.csf = csf;
                openList.add(next);
            }
        }

        openList.delete(node);
        closedList.add(node);

        if (closedList.has(target)) {
            return target.csf;
        }
    }
}

(() => {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/);
    const grid = lines.map((line) => line.split('').map((node) => new Node(Number(node) - 1)));

    let fullGrid = grid.map((row) => [...Array(5).keys()].map((i) => row.map((node) => new Node((node.cost + i) % 9))).flat());
    fullGrid = [...Array(5).keys()].map((i) => fullGrid.map((row) => row.map((node) => new Node((node.cost + i) % 9)))).flat();

    console.log('Part 1: ', findShortestPath(grid));
    console.log('Part 2: ', findShortestPath(fullGrid));
})();
