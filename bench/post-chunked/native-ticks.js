"use strict";

// Resolve native addon PCs that V8's built-in report leaves unaccounted.
const fs = require("node:fs");
const path = require("node:path");
const dir = process.argv[2];
const maps = fs.readFileSync(path.join(dir, "maps.txt"), "utf8")
    .split("\n").filter(line => line.endsWith("/n0.node"))
    .map(line => {
        const fields = line.trim().split(/\s+/);
        const [start, end] = fields[0].split("-").map(x => parseInt(x, 16));
        return { start, end, offset: parseInt(fields[2], 16) };
    });
const symbols = fs.readFileSync(path.join(dir, "addon-symbols.txt"), "utf8")
    .split("\n").map(line => line.match(/^([\da-f]+) [tT] (.+)$/))
    .filter(Boolean).map(match => ({ address: parseInt(match[1], 16), name: match[2] }));
const counts = new Map();
let total = 0, native = 0;
for (const line of fs.readFileSync(path.join(dir, "v8.log"), "utf8").split("\n")) {
    if (!line.startsWith("tick,")) continue;
    total++;
    const pc = Number(line.split(",")[1]);
    const map = maps.find(m => pc >= m.start && pc < m.end);
    if (!map) continue;
    native++;
    // ELF virtual addresses can differ from file offsets (Rust/lld LOAD segments).
    const base = maps.find(m => m.offset === 0).start;
    const relative = pc - base;
    let low = 0, high = symbols.length;
    while (low < high) {
        const mid = (low + high) >>> 1;
        if (symbols[mid].address <= relative) low = mid + 1;
        else high = mid;
    }
    const name = symbols[low - 1]?.name || "unknown addon symbol";
    counts.set(name, (counts.get(name) || 0) + 1);
}
console.log(`total_ticks=${total} addon_ticks=${native} addon_percent=${(100 * native / total).toFixed(2)}`);
for (const [name, count] of [...counts].sort((a, b) => b[1] - a[1]).slice(0, 15)) {
    console.log(`${count}\t${(100 * count / total).toFixed(2)}%\t${name}`);
}
