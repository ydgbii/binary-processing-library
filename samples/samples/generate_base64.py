
import base64
import re
import subprocess
from pathlib import Path

samples = Path(__file__).resolve().parent
source = samples / "sample.c"
object_file = samples / "sample.o"

subprocess.run(
    [
        "clang",
        "-target", "arm64-apple-macos",
        "-O2",
        "-c", str(source),
        "-o", str(object_file),
    ],
    check=True,
)

result = subprocess.run(
    ["xcrun", "llvm-objdump", "--disassemble", str(object_file)],
    capture_output=True,
    text=True,
    check=True,
)

instructions = []

for line in result.stdout.splitlines():
    match = re.match(
        r"^\s*[0-9a-fA-F]+:\s+([0-9a-fA-F]{8})\s+(add|sub)\s",
        line,
        re.IGNORECASE,
    )

    if match:
        word = int(match.group(1), 16)
        instructions.extend(word.to_bytes(4, "little"))

encoded = base64.b64encode(bytes(instructions)).decode("ascii")

print("Extracted instructions:", len(instructions) // 4)
print("Base64:", encoded)

