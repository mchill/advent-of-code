import * as fs from 'fs';
import * as _ from 'lodash';

const amphipodMapping: { [key: string]: number } = { 'A': 2, 'B': 4, 'C': 6, 'D': 8 };
const roomMapping: { [key: number]: string } = { 2: 'A', 4: 'B', 6: 'C', 8: 'D' };
const costs: { [key: string]: number } = { 'A': 1, 'B': 10, 'C': 100, 'D': 1000 };

function moveHallwayToRoom(oldHallways: string[], oldRooms: string[][], from: number, to: number): [string[], string[][], number] {
    const hallways = _.cloneDeep(oldHallways);
    const rooms = _.cloneDeep(oldRooms);

    const amphipod = hallways[from];
    const room = rooms[to];
    hallways[from] = null;
    let moves = Math.abs(from - to) + 1;

    for (let i = room.length - 1; i >= 0; i--) {
        if (room[i] == null) {
            room[i] = amphipod;
            moves += i;
            break;
        }
    }

    const cost = moves * costs[amphipod];
    return [hallways, rooms, cost];
}

function moveRoomToHallway(oldHallways: string[], oldRooms: string[][], from: number, to: number): [string[], string[][], number] {
    const hallways = _.cloneDeep(oldHallways);
    const rooms = _.cloneDeep(oldRooms);

    const room = rooms[from];
    let moves = Math.abs(from - to) + 1;
    let amphipod: string;

    for (let i = 0; i < room.length; i++) {
        if (room[i] != null) {
            amphipod = room[i];
            room[i] = null;
            moves += i;
            break;
        }
    }

    hallways[to] = amphipod;
    let cost = moves * costs[amphipod];
    return [hallways, rooms, cost];
}

const states = new Map<string, number>();

function step(hallways: string[], rooms: string[][], costSoFar: number): number {
    const state = hallways.join(',') + ',' + rooms.flat().join(',');
    if (states.has(state) && costSoFar >= states.get(state)) {
        return Infinity;
    }
    states.set(state, costSoFar);

    // Every amphipod is where it belongs
    if (rooms.every((room, roomIndex) => room == null || room.every((space) => space === roomMapping[roomIndex]))) {
        return 0;
    }

    for (let hallway = 0; hallway < hallways.length; hallway++) {
        const amphipod = hallways[hallway];
        const roomIndex = amphipodMapping[amphipod];
        const room = rooms[roomIndex];
        const path = hallways.slice(Math.min(hallway, roomIndex), Math.max(hallway, roomIndex) + 1);

        // No amphipod in space
        if (amphipod == null) {
            continue;
        }

        // Target room still has wrong amphipods
        if (room.some((space) => space !== amphipod && space != null)) {
            continue;
        }

        // Path not clear to target room
        if (path.filter((space) => space != null).length > 1) {
            continue;
        }

        const [newHallways, newRooms, cost] = moveHallwayToRoom(hallways, rooms, hallway, roomIndex);
        return cost + step(newHallways, newRooms, costSoFar + cost);
    }

    let minCost = Infinity;

    for (let roomIndex = 2; roomIndex < rooms.length - 2; roomIndex += 2) {
        // No wrong amphipods to move out of room
        if (rooms[roomIndex].every((space) => space === roomMapping[roomIndex] || space == null)) {
            continue;
        }

        for (let hallway = 0; hallway < hallways.length; hallway++) {
            const path = hallways.slice(Math.min(hallway, roomIndex), Math.max(hallway, roomIndex) + 1);

            // Hallway attached to room
            if (Object.values(amphipodMapping).includes(hallway)) {
                continue;
            }

            // Path not clear to hallway
            if (path.some((space) => space != null)) {
                continue;
            }

            let [newHallways, newRooms, cost] = moveRoomToHallway(hallways, rooms, roomIndex, hallway);
            cost += step(newHallways, newRooms, costSoFar + cost);

            minCost = Math.min(minCost, cost);
        }
    }

    return minCost;
}

function getMap(): [string[], string[][]] {
    const lines = fs.readFileSync(process.argv[2], 'utf-8').toString().trim().split(/\r?\n/).slice(2, -1);

    const hallways: string[] = Array(11).fill(null);
    const rooms: string[][] = Array(11).fill(null);

    rooms[2] = [lines[0][3], lines[1][3]];
    rooms[4] = [lines[0][5], lines[1][5]];
    rooms[6] = [lines[0][7], lines[1][7]];
    rooms[8] = [lines[0][9], lines[1][9]];

    return [hallways, rooms];
}

function part1() {
    const [hallways, rooms] = getMap();
    return step(hallways, rooms, 0);
}

function part2() {
    const [hallways, rooms] = getMap();

    rooms[2].splice(1, 0, 'D', 'D');
    rooms[4].splice(1, 0, 'C', 'B');
    rooms[6].splice(1, 0, 'B', 'A');
    rooms[8].splice(1, 0, 'A', 'C');

    return step(hallways, rooms, 0);
}

(() => {
    console.log('Part 1: ', part1());
    console.log('Part 2: ', part2());
})();
