#!/usr/bin/env bash
# ------------------------------------------------------------
# Compare the consistency of two image generation flows for every example
# under examples/peripherals:
#   (Flow A) cargo objcopy -> (bin) -> cargo xtask gen-image
#   (Flow B) cargo xtask elf2img  (ELF -> image directly)
# Then check whether the resulting .img files are byte-identical; if not,
# print a short diff summary.
#
# Reference manual steps (original user workflow):
#   cargo build -p gpio-button-demo --target riscv64gc-unknown-none-elf --release
#   cargo objcopy -p gpio-button-demo --release --target riscv64gc-unknown-none-elf -- -O binary target/.../gpio-button-demo.objcopy.bin
#   cargo xtask gen-image -i target/.../gpio-button-demo.objcopy.bin -o target/.../gpio-button-demo.gen-image.img
#   cargo xtask elf2img  -i target/.../gpio-button-demo -o target/.../gpio-button-demo.elf2img.img
#   cmp -x gen-image.img elf2img.img
#
# Usage:
#   scripts/test-image-flows.sh             # Release build (default)
#   scripts/test-image-flows.sh --debug     # Use debug build
#   scripts/test-image-flows.sh --keep-temp # Keep intermediate artifacts (for debugging)
#
# Dependencies:
#   1. cargo-binutils installed:  cargo install cargo-binutils && rustup component add llvm-tools-preview
#   2. xtask subcommands available (xtask crate in workspace)
#   3. cmp / hexdump / stat (available on macOS / most *nix)
# ------------------------------------------------------------
set -euo pipefail

MODE=release     # or debug
KEEP_TEMP=0
TARGET=riscv64gc-unknown-none-elf
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLE_DIR="$ROOT_DIR/examples/peripherals"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug) MODE=debug; shift ;;
    --keep-temp) KEEP_TEMP=1; shift ;;
    -h|--help)
      grep '^# ' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
  *) echo "Unknown argument: $1" >&2; exit 1 ;;
  esac
done

if ! command -v cargo objcopy >/dev/null 2>&1; then
  echo "[ERR] 'cargo objcopy' not found. Install cargo-binutils and enable llvm-tools-preview" >&2
  echo "      cargo install cargo-binutils && rustup component add llvm-tools-preview" >&2
  exit 1
fi

BUILD_FLAG=(--release)
[[ $MODE == debug ]] && BUILD_FLAG=()

# Collect all example package names (parse each Cargo.toml 'name' field)
PACKAGES=()
while IFS= read -r -d '' toml; do
  name=$(grep -E '^name\s*=\s*"' "$toml" | head -1 | cut -d '"' -f2)
  [[ -n $name ]] && PACKAGES+=("$name")
done < <(find "$EXAMPLE_DIR" -mindepth 1 -maxdepth 2 -name Cargo.toml -print0 | sort -z)

if [[ ${#PACKAGES[@]} -eq 0 ]]; then
  echo "[ERR] No example packages found." >&2
  exit 1
fi

echo "[INFO] Testing example packages: ${PACKAGES[*]} (mode=$MODE, target=$TARGET)"

echo
printf '%-25s %-8s %-8s %-8s %-8s %-6s\n' "Package" "BIN" "IMG(A)" "IMG(B)" "SHA(A)" "Match"
printf '%0.s-' {1..80}; echo

FAILED=0
DETAIL_REPORT=""

for pkg in "${PACKAGES[@]}"; do
  echo "[STEP] Build $pkg ..." >&2
  cargo build -p "$pkg" --target "$TARGET" "${BUILD_FLAG[@]}" >/dev/null

  ELF_PATH="$ROOT_DIR/target/$TARGET/$MODE/$pkg"
  if [[ ! -f "$ELF_PATH" ]]; then
  echo "[ERR] ELF not found: $ELF_PATH" >&2
    FAILED=$((FAILED+1))
    continue
  fi

  # Artifact naming
  BIN_A="$ELF_PATH.objcopy.bin"
  IMG_A="$ELF_PATH.gen-image.img"   # Flow A image
  IMG_B="$ELF_PATH.elf2img.img"     # Flow B image

  echo "[STEP] objcopy -> bin ($pkg)" >&2
  cargo objcopy -p "$pkg" "${BUILD_FLAG[@]}" --target "$TARGET" -- -O binary "$BIN_A"

  echo "[STEP] Flow A: gen-image ($pkg)" >&2
  cargo xtask gen-image -i "$BIN_A" -o "$IMG_A" >/dev/null

  echo "[STEP] Flow B: elf2img ($pkg)" >&2
  cargo xtask elf2img -i "$ELF_PATH" -o "$IMG_B" >/dev/null || { echo "[ERR] elf2img failed" >&2; FAILED=$((FAILED+1)); continue; }

  if [[ ! -f "$IMG_A" || ! -f "$IMG_B" ]]; then
  echo "[ERR] Missing output file(s) ($pkg)" >&2
    FAILED=$((FAILED+1))
    continue
  fi

  SIZE_BIN=$(stat -f %z "$BIN_A")
  SIZE_IMGA=$(stat -f %z "$IMG_A")
  SIZE_IMGB=$(stat -f %z "$IMG_B")
  SHA_A=$(shasum -a 256 "$IMG_A" | cut -d' ' -f1 | cut -c1-7)
  SHA_B=$(shasum -a 256 "$IMG_B" | cut -d' ' -f1 | cut -c1-7)

  if cmp -s "$IMG_A" "$IMG_B"; then
    MATCH=YES
  else
    MATCH=NO
    FAILED=$((FAILED+1))
  # Produce a short difference summary
    diff_hex=$( (hexdump -Cv "$IMG_A" | head -n 8) ; echo '---'; (hexdump -Cv "$IMG_B" | head -n 8) )
  DETAIL_REPORT+=$'\n==== Difference: '"$pkg"$' ===='"\n"
    DETAIL_REPORT+=$'ELF: '"$ELF_PATH"$'\n'
    DETAIL_REPORT+=$'BIN size: '"$SIZE_BIN"$'  IMG(A) size: '"$SIZE_IMGA"$'  IMG(B) size: '"$SIZE_IMGB"$'\n'
    DETAIL_REPORT+=$'SHA(A): '"$SHA_A"$'  SHA(B): '"$SHA_B"$'\n'
  DETAIL_REPORT+=$'First 8 hex lines (A then B):\n'"$diff_hex"$'\n'
  fi

  printf '%-25s %-8s %-8s %-8s %-8s %-6s\n' "$pkg" "$SIZE_BIN" "$SIZE_IMGA" "$SIZE_IMGB" "$SHA_A" "$MATCH"

  # Cleanup intermediates (unless keeping)
  if [[ $KEEP_TEMP -eq 0 ]]; then
    rm -f "$BIN_A"
    [[ $MATCH == YES ]] && rm -f "$IMG_A" "$IMG_B"
  fi

done

printf '%0.s-' {1..80}; echo
if [[ $FAILED -eq 0 ]]; then
  echo "[RESULT] All examples: Flow A and Flow B images match exactly."
else
  echo "[RESULT] $FAILED example(s) mismatched, see diff summary below." >&2
  echo "$DETAIL_REPORT" >&2
  exit 2
fi
