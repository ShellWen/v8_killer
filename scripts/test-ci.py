import os
from pathlib import Path
import ast
import shutil
import subprocess
import tempfile
from types import SimpleNamespace
from unittest.mock import Mock, patch


ROOT = Path(__file__).resolve().parent.parent
bash = os.environ.get("CI_BASH", "bash")
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)
    tools = root / "bin"
    tools.mkdir()
    for name in ("cargo", "python", "curl", "tar", "sha256sum"):
        tool = tools / name
        tool.write_text('''#!/usr/bin/env bash
set -eu
echo "$(basename "$0") $*" >> "$CALLS"
case $(basename "$0") in
  curl)
    output=${@: -1}
    mkdir -p "$(dirname "$output")"
    if [[ $output == *SHASUMS256.txt ]]; then
      printf 'fake  win-x64/node.exe\nfake  node-v22.23.2-linux-x64.tar.xz\nfake  node-v24.21.0-linux-x64.tar.xz\nfake  node-v26.8.1-linux-x64.tar.xz\n' > "$output"
    else touch "$output"; fi
    ;;
  sha256sum) cat >/dev/null; exit "${CHECKSUM_EXIT:-0}" ;;
esac
''', encoding="utf-8", newline="\n")
        tool.chmod(0o755)
    calls = root / "calls"
    env = {**os.environ, "PATH": f"{tools}{os.pathsep}{os.environ['PATH']}", "CALLS": calls.as_posix()}
    shutil.copytree(ROOT / "scripts", root / "scripts", ignore=shutil.ignore_patterns("__pycache__", "local"))
    for target in ("x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"):
        for profiles in ("debug", "debug release"):
            calls.write_text("")
            subprocess.run([bash, "scripts/ci-build.sh", target, profiles], cwd=root, env=env, check=True)
            lines = calls.read_text().splitlines()
            count = len(profiles.split())
            assert sum(line.startswith("cargo build --locked") for line in lines) == count
            tests = [line for line in lines if line.startswith("python scripts/test-node.py")]
            assert len(tests) == 3 * count
            assert all("--wine" not in line for line in tests)
            assert all(("--file-output" in line) == ("windows" in target) for line in tests)
            for profile in profiles.split():
                build = next(i for i, line in enumerate(lines) if line.startswith("cargo build ") and ("--release" in line) == (profile == "release"))
                assert all(lines.index(line) > build for line in tests if f"/{profile}/" in line)
                assert sum(f"/{profile}/" in line for line in tests) == 3
            assert sum(line.startswith("sha256sum") for line in lines) == 3 * count
            assert lines[-1].endswith("--all-targets --all-features -- -D warnings")
    calls.write_text("")
    result = subprocess.run([bash, "scripts/ci-node.sh", "win", "debug", "x86_64-pc-windows-msvc"], cwd=root,
                            env={**env, "CHECKSUM_EXIT": "1"})
    assert result.returncode != 0
    assert "python" not in calls.read_text()

source = ast.parse((ROOT / "scripts/test-node.py").read_text(encoding="utf-8"))
definitions = ast.Module(body=[node for node in source.body if isinstance(node, (ast.Import, ast.ImportFrom, ast.FunctionDef))], type_ignores=[])
scope = {}
exec(compile(definitions, "test-node.py", "exec"), scope)
for platform in ("nt", "posix"):
    scope["os"] = SimpleNamespace(name=platform, environ={}, killpg=Mock())
    scope["signal"] = SimpleNamespace() if platform == "nt" else SimpleNamespace(SIGKILL=object())
    process = Mock(pid=123, returncode=-9)
    process.__enter__ = Mock(return_value=process)
    process.__exit__ = Mock(return_value=False)
    for file_output in (False, True):
        scope["args"] = SimpleNamespace(file_output=file_output, wine=False)
        process.wait.side_effect = [subprocess.TimeoutExpired("node", 30), None]
        process.communicate.side_effect = [subprocess.TimeoutExpired("node", 30), ("测试😀".encode(), b"")]
        scope["os"].killpg.reset_mock()
        with patch.object(subprocess, "Popen", return_value=process) as popen, patch.object(subprocess, "run") as run:
            result = scope["run"](["node"])
            assert result["timeout"]
            assert popen.call_args.kwargs["start_new_session"] == (platform != "nt")
            if platform == "nt":
                assert run.call_args.args[0] == ["taskkill", "/PID", "123", "/T", "/F"]
                scope["os"].killpg.assert_not_called()
            else:
                run.assert_not_called()
                scope["os"].killpg.assert_called_once_with(process.pid, scope["signal"].SIGKILL)
            if not file_output:
                assert result["stdout"] == "测试😀"
    path = r"C:\space 测试😀\node.exe"
    assert scope["target_path"](path) == path
    scope["args"].wine = True
    with patch.object(subprocess, "check_output", return_value=path + "\n") as winepath:
        assert scope["target_path"]("/unix/node.exe") == path
        assert winepath.call_args.args[0] == ["winepath", "-w", "/unix/node.exe"]
print("CI orchestration, checksum failure, native/Wine paths and timeout checks passed")
