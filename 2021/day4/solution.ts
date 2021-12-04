import * as fs from 'fs';

class Board {
    public board: (string | boolean)[][];

    public constructor(board: string[][]) {
        this.board = board;
    }

    public fill(value: string) {
        for (let i = 0; i < this.board.length; i++) {
            for (let j = 0; j < this.board[0].length; j++) {
                if (this.board[i][j] === value) {
                    this.board[i][j] = true;
                }
            }
        }
    }

    public check() {
        const indices = [...Array(5).keys()];
        return indices.some((row) => indices.every((column) => this.board[row][column] === true)) ||
            indices.some((column) => indices.every((row) => this.board[row][column] === true));
    }

    public sumRemaining() {
        return this.board.flat().filter((value) => value !== true).reduce((a, b) => a + Number(b), 0);
    }
}

async function solve(findLoser: boolean = false) {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().split(/\r?\n/).slice(0, -1);
    const draw = lines[0].split(',');

    let boards: Board[] = [];
    for (let i = 2; i < lines.length; i += 6) {
        boards.push(new Board(lines.slice(i, i + 5).map((line) => line.split(' ').filter((value) => value !== ''))));
    }

    for (const value of draw) {
        for (let i = boards.length - 1; i >= 0; i--) {
            const board = boards[i];
            board.fill(value);
            if (board.check()) {
                if (findLoser) {
                    boards.splice(i, 1);
                }
                if (!findLoser || boards.length === 0) {
                    return board.sumRemaining() * Number(value);
                }
            }
        }
    }
}

(async () => {
    console.log('Part 1: ', await solve());
    console.log('Part 2: ', await solve(true));
})();
