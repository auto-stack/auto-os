// src/back/api.ts — Klondike API client & storage glue
export interface GameRecord {
    best_moves: number;
    best_time_s: number;
    games_won: number;
    games_played: number;
}

const STORAGE_KEY = 'klondike_game_records';

function getLocalRecords(): GameRecord {
    try {
        const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(STORAGE_KEY) : null;
        if (raw) return JSON.parse(raw);
    } catch (_) {}
    return { best_moves: 0, best_time_s: 0, games_won: 0, games_played: 0 };
}

function saveLocalRecords(rec: GameRecord) {
    try {
        if (typeof localStorage !== 'undefined') {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(rec));
        }
    } catch (_) {}
}

export async function get_records(): Promise<GameRecord> {
    try {
        const res = await fetch('/api/records');
        if (res.ok) {
            const data = await res.json();
            saveLocalRecords(data);
            return data;
        }
    } catch (_) {}
    return getLocalRecords();
}

export async function record_win(moves: number, time_s: number): Promise<GameRecord> {
    const rec = getLocalRecords();
    rec.games_won += 1;
    rec.games_played += 1;
    if (rec.best_moves === 0 || moves < rec.best_moves) rec.best_moves = moves;
    if (rec.best_time_s === 0 || time_s < rec.best_time_s) rec.best_time_s = time_s;
    saveLocalRecords(rec);
    try {
        const res = await fetch('/api/records/win', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ moves, time_s }),
        });
        if (res.ok) return await res.json();
    } catch (_) {}
    return rec;
}

export async function record_start(): Promise<GameRecord> {
    const rec = getLocalRecords();
    rec.games_played += 1;
    saveLocalRecords(rec);
    try {
        const res = await fetch('/api/records/start', { method: 'POST' });
        if (res.ok) return await res.json();
    } catch (_) {}
    return rec;
}
