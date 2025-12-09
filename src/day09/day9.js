import { readFileSync } from "node:fs";

function area(a, b) {
  return (1 + Math.abs(b[1] - a[1])) * (1 + Math.abs(b[0] - a[0]));
}

function is_inside(a, b, p) {
  const left = Math.min(a[0], b[0]) + 1;
  const right = Math.max(a[0], b[0]) - 1;
  const top = Math.max(a[1], b[1]) - 1;
  const bottom = Math.min(a[1], b[1]) + 1;

  return left <= p[0] && right >= p[0] && top >= p[1] && bottom <= p[1];
}

function get_points_on_line(a, b) {
  const points = [];
  if (a[0] == b[0]) {
    const min = Math.min(a[1], b[1]);
    const max = Math.max(a[1], b[1]);
    for (let i = min; i <= max; i++) {
      points.push([a[0], i]);
    }
  } else {
    const min = Math.min(a[0], b[0]);
    const max = Math.max(a[0], b[0]);
    for (let i = min; i <= max; i++) {
      points.push([i, a[1]]);
    }
  }

  return points;
}

const file = './day9.txt';
// const file = "./sample.txt";
const content = readFileSync(file, "utf-8");

const points = content
  .split("\n")
  .filter((l) => l.length > 0)
  .map((l) => l.split(",").map((n) => parseInt(n, 10)));

const edges = [];

for (let i = 1; i < points.length; i++) {
  edges.push([points[i - 1], points[i]]);
}
edges.push([points[0], points[points.length - 1]]);

const points_on_edges = [];
for (const edge of edges) {
  const points = get_points_on_line(edge[0], edge[1]);
  for (const p of points) {
    points_on_edges.push(p);
  }
}

const areas = [];

for (let i = 0; i < points.length; i++) {
  for (let j = i + 1; j < points.length; j++) {
    areas.push([points[i], points[j], area(points[i], points[j])]);
  }
}

areas.sort((a, b) => b[2] - a[2]);

for (const area of areas) {
  let found = true;

    for (const p of points_on_edges) {
      if (is_inside(area[0], area[1], p)) {
        found = false;
        break;
      }
    }

  if (found) {
    console.log('found largest', area[2]);
    break;
  }
}
