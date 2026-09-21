import os
from pathlib import Path
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parent.parent
with tempfile.TemporaryDirectory() as directory:
    root = Path(directory)
    tools = root / "bin"
    tools.mkdir()
    for name in ("cargo", "python3", "curl", "tar", "sha256sum"):
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
''')
        tool.chmod(0o755)
    calls = root / "calls"
    env = {**os.environ, "PATH": f"{tools}:{os.environ['PATH']}", "CALLS": str(calls)}
    (root / "scripts").symlink_to(ROOT / "scripts", target_is_directory=True)
    for target in ("x86_64-unknown-linux-gnu", "x86_64-pc-windows-gnu"):
        for profiles in ("debug", "debug release"):
            calls.write_text("")
            subprocess.run(["bash", "scripts/ci-build.sh", target, profiles], cwd=root, env=env, check=True)
            lines = calls.read_text().splitlines()
            count = len(profiles.split())
            assert sum(line.startswith("cargo build --locked") for line in lines) == count
            tests = [line for line in lines if line.startswith("python3 scripts/test-node.py")]
            assert len(tests) == 3 * count
            assert all(("--wine --file-output" in line) == ("windows" in target) for line in tests)
            assert sum(line.startswith("sha256sum") for line in lines) == 3 * count
            assert lines[-1].endswith("--all-targets --all-features -- -D warnings")
    calls.write_text("")
    result = subprocess.run(["bash", "scripts/ci-node.sh", "win", "debug", "x86_64-pc-windows-gnu"], cwd=root,
                            env={**env, "CHECKSUM_EXIT": "1"})
    assert result.returncode != 0
    assert "python3" not in calls.read_text()
print("CI orchestration and checksum failure checks passed")
