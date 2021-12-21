import * as fs from 'fs';

function roll(die: number) {
    return (die - 1) % 100 + 1;
}

function move(position: number, steps: number) {
    return (position - 1 + steps) % 10 + 1;
}

function part1() {
    const players = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/).map((line) => Number(line.split(': ')[1]));

    let [p1, p2] = players;
    let [s1, s2] = [0, 0];
    let die = 0;

    while (true) {
        p1 = move(p1, roll(++die) + roll(++die) + roll(++die));
        s1 += p1;
        if (s1 >= 1000) {
            return s2 * die;
        }

        p2 = move(p2, roll(++die) + roll(++die) + roll(++die));
        s2 += p2;
        if (s2 >= 1000) {
            return s1 * die;
        }
    }
}

const memoizedRounds: { [key: string]: number[] } = {};

function round(players: number[], scores: number[], current: number) {
    const memoizationKey = `${players[0]},${players[1]},${scores[0]},${scores[1]},${current}`;
    if (memoizationKey in memoizedRounds) {
        return memoizedRounds[memoizationKey];
    }

    const rolls = { 3: 1, 4: 3, 5: 6, 6: 7, 7: 6, 8: 3, 9: 1 };
    const wins = [0, 0];

    for (const [roll, probability] of Object.entries(rolls)) {
        let position = players[current];
        let score = scores[current];

        position = move(position, Number(roll));
        score += position;

        if (score >= 21) {
            wins[current] += probability;
            continue;
        }

        const newPlayers = [...players];
        newPlayers[current] = position;
        const newScores = [...scores];
        newScores[current] = score;

        const subWins = round(newPlayers, newScores, (current + 1) % 2);
        wins[0] += subWins[0] * probability;
        wins[1] += subWins[1] * probability;
    }

    memoizedRounds[`${players[0]},${players[1]},${scores[0]},${scores[1]},${current}`] = wins;
    return wins;
}

function part2() {
    const players = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/).map((line) => Number(line.split(': ')[1]));

    const wins = round(players, [0, 0], 0);
    return Math.max(...wins);
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
