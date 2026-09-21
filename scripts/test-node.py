import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import signal


if hasattr(sys.stdout, "reconfigure"):
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")

parser = argparse.ArgumentParser()
parser.add_argument("launcher")
parser.add_argument("node")
parser.add_argument("--wine", action="store_true")
parser.add_argument("--report", type=Path)
parser.add_argument("--file-output", action="store_true")
args = parser.parse_args()
launcher = str(Path(args.launcher).resolve())
node = str(Path(args.node).resolve())
prefix = ["wine"] if args.wine else []
report = {"runs": [], "artifacts": {}, "capture": "file" if args.file_output else "pipe"}
for path in (launcher, node, str(Path(launcher).with_name("v8_killer_core.dll" if args.wine or os.name == "nt" else "libv8_killer_core.so"))):
    report["artifacts"][path] = hashlib.sha256(Path(path).read_bytes()).hexdigest()

def target_path(path):
    return subprocess.check_output(["winepath", "-w", str(path)], encoding="utf-8", timeout=30).strip() if args.wine else str(path)

def kill_tree(process):
    if os.name == "nt":
        subprocess.run(["taskkill", "/PID", str(process.pid), "/T", "/F"],
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=10, check=True)
    else:
        os.killpg(process.pid, signal.SIGKILL)

def run(command):
    if args.file_output:
        with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr:
            with subprocess.Popen(command, stdout=stdout, stderr=stderr, start_new_session=os.name != "nt",
                                  env={**os.environ, "NO_COLOR": "1"}) as process:
                timed_out = False
                try:
                    process.wait(timeout=30)
                except subprocess.TimeoutExpired:
                    timed_out = True
                    kill_tree(process)
                    process.wait()
            stdout.seek(0)
            stderr.seek(0)
            return {"command": command, "returncode": process.returncode, "timeout": timed_out,
                    "stdout": stdout.read().decode("utf-8", errors="replace"),
                    "stderr": stderr.read().decode("utf-8", errors="replace")}
    with subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                          start_new_session=os.name != "nt", env={**os.environ, "NO_COLOR": "1"}) as process:
        timed_out = False
        try:
            stdout, stderr = process.communicate(timeout=30)
        except subprocess.TimeoutExpired:
            timed_out = True
            kill_tree(process)
            stdout, stderr = process.communicate()
    return {"command": command, "returncode": process.returncode, "timeout": timed_out,
            "stdout": stdout.decode("utf-8", errors="replace"), "stderr": stderr.decode("utf-8", errors="replace")}

metadata = run([*prefix, node, "-p", "JSON.stringify({node:process.version,v8:process.versions.v8,platform:process.platform,arch:process.arch})"])
report["metadata"] = metadata
version = metadata["stdout"].strip()
with tempfile.TemporaryDirectory(prefix="v8-killer-") as directory:
    root = Path(directory)
    config = root / "replace.toml"
    config.write_text('''[rules.unicode]
matcher = { type = "resource-name-regexp", regexp = "unicode-(测试|%E6%B5%8B%E8%AF%95)" }
processors = [{ type = "replace", from = "原始😀", to = "替换成功🚀" }]
''', encoding="utf-8")
    no_match = root / "no-match.toml"
    no_match.write_text(config.read_text(encoding="utf-8").replace("unicode-(测试|%E6%B5%8B%E8%AF%95)", "NEVER_MATCH_92cfa"), encoding="utf-8")

    def write(name, source):
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(source, encoding="utf-8")
        return path

    cjs_source = 'console.log("RESULT:原始😀\\u0000尾");\n// literal NUL: \0\nconsole.log("RESULT:after-nul");\n'
    cjs = write("unicode-测试.cjs", cjs_source)
    js = write("unicode-测试.js", cjs_source)
    write("unicode-测试-dep.mjs", 'export let value = "原始😀\\u0000尾"; export function update() { value = "live"; }\n// NUL: \0\n')
    write("unmatched.mjs", 'export default "原始😀";\n')
    esm = write("unicode-测试.mjs", '''import assert from "node:assert/strict";
import { value, update } from "./unicode-测试-dep.mjs";
import untouched from "./unmatched.mjs";
import "./unicode-测试.cjs";
console.log("RESULT:" + value);
update();
assert.equal(value, "live");
assert.equal(untouched, "原" + "始😀");
assert.ok(import.meta.url.endsWith(".mjs"));
await Promise.resolve();
console.log("RESULT:原始😀");
''')
    write("module/package.json", '{"type":"module"}\n')
    write("module/unicode-测试-dep.js", 'export default "原始😀";\n')
    module_js = write("module/unicode-测试.js", '''import value from "./unicode-测试-dep.js";
console.log("RESULT:" + value);
console.log("RESULT:原始😀");
''')
    dynamic = write("dynamic.cjs", '''import("./unicode-测试.mjs").then(() => console.log("RESULT:dynamic"));
''')
    require_esm = write("require.cjs", '''const { value, update } = require("./unicode-测试-dep.mjs");
console.log("RESULT:" + value);
update();
''')
    exit_module = write("unicode-测试-exit.mjs", 'console.log("RESULT:原始😀"); process.exitCode = 23;\n')
    unmatched = write("unmatched-entry.mjs", 'console.log("RESULT:原始😀");\n')
    cjs_deps = write("deps.cjs", 'require("./unicode-测试.cjs"); require("./unicode-测试.js");\n')
    spaced = write("space 测试😀/unicode-测试 🚀.mjs", 'console.log("RESULT:原始😀");\n')
    arguments = ["space value", "测试😀", "", 'a"b', "trailing\\", "space trailing\\"]
    argv = write("argv.cjs", 'console.log("RESULT:" + JSON.stringify(process.argv.slice(2))); process.exitCode = 23;\n')
    cases = [
        (cjs, ["原始😀\0尾", "after-nul"], 0, True),
        (js, ["原始😀\0尾", "after-nul"], 0, True),
        (esm, ["原始😀\0尾", "after-nul", "原始😀\0尾", "原始😀"], 0, True),
        (module_js, ["原始😀", "原始😀"], 0, True),
        (dynamic, ["原始😀\0尾", "after-nul", "原始😀\0尾", "原始😀", "dynamic"], 0, True),
        (require_esm, ["原始😀\0尾"], 0, True),
        (exit_module, ["原始😀"], 23, True),
        (unmatched, ["原始😀"], 0, False),
        (cjs_deps, ["原始😀\0尾", "after-nul"] * 2, 0, True),
        (spaced, ["原始😀"], 0, True),
        (argv, [json.dumps(arguments, ensure_ascii=False, separators=(",", ":"))], 23, False),
    ]
    for entry, expected, exit_code, matched in cases:
        for mode in ("baseline", "no-match", "match"):
            command = [launcher, "--config", target_path(config if mode == "match" else no_match), target_path(node), "--"] if mode != "baseline" else [node]
            result = run([*prefix, *command, target_path(entry), *(arguments if entry == argv else [])])
            values = [line.removeprefix("RESULT:") for line in result["stdout"].splitlines() if line.startswith("RESULT:")]
            wanted = [value.replace("原始😀", "替换成功🚀") for value in expected] if mode == "match" and matched else expected
            result.update(case=str(entry.relative_to(root)), mode=mode, expected=wanted,
                          passed=not result["timeout"] and result["returncode"] == exit_code and values == wanted)
            report["runs"].append(result)
            print(f"{version}: {entry.relative_to(root)} {mode}: {'PASS' if result['passed'] else 'FAIL'}", flush=True)
            if args.report:
                args.report.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
sys.exit(0 if all(result["passed"] for result in report["runs"]) else 1)
