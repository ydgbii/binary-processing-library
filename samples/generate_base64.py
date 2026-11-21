
import base64
import re
import subprocess
from pathlib import Path

samples = Path(__file__).resolve().parent
source = samples / "sample.c"
object_file = samples / "sample.o"


def run(*args):
    return subprocess.run(
        args,
        capture_output=True,
        text=True,
        check=True,
    ).stdout


# Compile C source into an ARM64 object file.
run(
    "clang", "-target", "arm64-apple-macos",
    "-O2", "-c", str(source), "-o", str(object_file)
)

# Read function addresses from the symbol table.
symbols = {}

for line in run("nm", str(object_file)).splitlines():
    match = re.match(
        r"^([0-9a-fA-F]+)\s+T\s+_(sample_add|sample_sub)$",
        line,
    )
    if match:
        symbols[match.group(2)] = int(match.group(1), 16)

if set(symbols) != {"sample_add", "sample_sub"}:
    raise RuntimeError("Required function symbols not found")

# Read disassembled instructions and their addresses.
instructions = {}

output = run(
    "xcrun", "llvm-objdump",
    "--disassemble", str(object_file)
)

for line in output.splitlines():
    match = re.match(
        r"^\s*([0-9a-fA-F]+):\s+([0-9a-fA-F]{8})\s+(\w+)",
        line,
    )
    if match:
        address = int(match.group(1), 16)
        word = int(match.group(2), 16)
        mnemonic = match.group(3).lower()
        instructions[address] = (word, mnemonic)

# Extract the first instruction of each selected function.
result = bytearray()

for name, expected in [
    ("sample_add", "add"),
    ("sample_sub", "sub"),
]:
    address = symbols[name]

    if address not in instructions:
        raise RuntimeError(f"No instruction found for {name}")

    word, mnemonic = instructions[address]

    if mnemonic != expected:
        raise RuntimeError(
            f"Expected {expected} in {name}, got {mnemonic}"
        )

    result.extend(word.to_bytes(4, "little"))

encoded = base64.b64encode(result).decode("ascii")

print("Extracted instructions:", len(result) // 4)
print("Base64:", encoded)
