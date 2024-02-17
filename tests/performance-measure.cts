import fs from "fs/promises";
import cp from "child_process";
import path from "path";

async function run(names: string[], repeat: number) {
  const summary: { [key: string]: any } = {
    relativeReplayGenTimeMean: 0,
  };
  for (let name of names) {
    summary[name] = {
      results: [],
      roundTripTimeMean: 0,
      replayGenTimeMean: 0,
      relativeReplayGenTimeMean: 0,
    };
  }
  for (let i = 0; i < repeat; i++) {
    for (let name of names) {
      cp.execSync(`npm t -- online -j -t ${name}`);
      const testPath = path.join("tests/online", name);
      const performancePath = path.join(testPath, "performance.ndjson");
      const tracePath = path.join(testPath, "benchmark/bin_0/trace-ref.r3");
      const jsons = await fs.readFile(performancePath, "utf8");
      const results = jsons.split("\n").filter((r) => r !== "");
      let parsed = {};
      for (let result of results) {
        const p = JSON.parse(result);
        parsed[p.name.replace(" ", "_")] = p;
      }
      const traceSize = (await fs.readFile(tracePath, "utf8")).split(
        "\n"
      ).length;
      let roundResult = {
        roundTripTime:
          parsed["round-trip_time"].duration -
          parsed["user_interaction"].duration,
        replayGenTime: parsed["rust-backend"].duration,
        traceSize,
        relativeReplayGenTime: parsed["rust-backend"].duration / traceSize,
      };
      summary[name].results.push(roundResult);
    }
  }
  for (let name of names) {
    for (let result of summary[name].results) {
      summary[name].roundTripTimeMean += result.roundTripTime;
      summary[name].replayGenTimeMean += result.replayGenTime;
      summary[name].relativeReplayGenTimeMean += result.relativeReplayGenTime;
    }
    summary[name].roundTripTimeMean /= repeat;
    summary[name].replayGenTimeMean /= repeat;
    summary[name].relativeReplayGenTimeMean /= repeat;
    summary.relativeReplayGenTimeMean +=
      summary[name].relativeReplayGenTimeMean;
  }
  summary.relativeReplayGenTimeMean /= names.length;
  await fs.writeFile("evaluation.json", JSON.stringify(summary));
}

const names = [
  "boa",
  "commanderkeen",
  "ffmpeg",
  "fib",
  "figma-startpage",
  "funky-kart",
  "game-of-life",
  "guiicons",
  "handy-tools",
  "heatmap",
  "jsc",
  "kittygame",
  "ogv",
  "pathfinding",
  "riconpacker",
  "rtexviewer",
  //   "sandspiel",
  "sqlgui",
  "video",
  "multiplyInt",
];
run(names, 5);
