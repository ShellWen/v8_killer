import os
from pathlib import Path
import subprocess
import sys
import tempfile


launcher = str(Path(sys.argv[1]).resolve())
node = sys.argv[2]
with tempfile.TemporaryDirectory(prefix="v8-killer-") as directory:
    root = Path(directory)
    script = root / "unicode-测试.cjs"
    script.write_text('console.log("原始😀\\u0000尾");\n// literal NUL: \0\nconsole.log("after-nul");\n', encoding="utf-8")
    config = root / "replace.toml"
    config.write_text('''[rules.unicode]
matcher = { type = "resource-name-keyword", keyword = "unicode-测试" }
processors = [
  { type = "replace", from = "原始😀", to = "替换成功🚀" },
  { type = "replace", from = "esm-original", to = "esm-replaced" },
]
''', encoding="utf-8")
    esm = root / "unicode-测试.mjs"
    esm.write_text('import "./unicode-测试.cjs";\nconsole.log("esm-original");\n', encoding="utf-8")
    version = subprocess.check_output([node, "--version"], text=True).strip()
    js = root / "unicode-测试.js"
    js.write_bytes(script.read_bytes())
    for entry in (script, js, esm):
        result = subprocess.run(
            [launcher, "--config", str(config), node, "--", str(entry)],
            capture_output=True, text=True, encoding="utf-8", timeout=30,
            env={**os.environ, "NO_COLOR": "1"},
        )
        output = result.stdout + result.stderr
        assert result.returncode == 0, output
        assert "替换成功🚀\0尾\n" in output, output
        assert "原始😀\0尾\n" not in output, output
        assert "source processing is disabled" not in output, output
        assert "after-nul\n" in output, output
        if entry == esm:
            assert "esm-original\n" in output, output
        print(f"{version}: {entry.suffix} default mode passed")
