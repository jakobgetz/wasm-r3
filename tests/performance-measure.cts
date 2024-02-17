import fs from "fs/promises";
import cp from "child_process";
import path from "path";

async function run(names: string[], repeat: number) {
  const summary: { [key: string]: any } = {};
  for (let name of names) {
    summary[name] = {
      results: [],
      roundTripTimeMean: 0
    };
  }
  for (let i = 0; i < repeat; i++) {
    for (let name of names) {
      cp.execSync(`npm t -- online -j -t ${name}`);
      const performancePath = path.join(
        "tests/online",
        name,
        "performance.ndjson"
      );
      const jsons = await fs.readFile(performancePath, "utf8");
      const results = jsons.split("\n").filter(r => r !== "");
      let parsed = {};
      for (let result of results) {
        const p = JSON.parse(result);
        parsed[p.name.replace(" ", "_")] = p;
      }
      let roundResult = {
        roundTripTime:
          parsed["round-trip_time"].duration -
          parsed["user_interaction"].duration,
        replayGenTime: parsed["rust-backend"].duration,
        traceSize: 
      };
      summary[name].results.push(roundResult);
    }
  }
  for (let name of names) {
    for (let result of summary[name].results) {
        summary[name].roundTripTimeMean += result.roundTripTime
    }
    summary[name].roundTripTimeMean /= repeat
  }

  await fs.writeFile('evaluation.json', JSON.stringify(summary));
}

const names = [
  "boa",
//   "commanderkeen",
//   "ffmpeg",
//   "fib",
//   "figma-startpage",
//   "funky-kart",
  "game-of-life",
//   "guiicons",
//   "handy-tools",
//   "heatmap",
//   "jsc",
//   "kittygame",
//   "ogv",
//   "pathfinding",
//   "riconpacker",
//   "rtexviewer",
// //   "sandspiel",
//   "sqlgui",
//   "video",
//   "multiplyInt",
];
run(names, 2);
