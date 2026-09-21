#!/usr/bin/env bash
set -euo pipefail

platform=$1
profile=$2
target=$3
mkdir -p .cache/node reports
for version in 22.23.2 24.21.0 26.8.1; do
  root=".cache/node/v${version}-${platform}-x64"
  mkdir -p "$root"
  curl --fail --location --retry 3 "https://nodejs.org/dist/v${version}/SHASUMS256.txt" -o "$root/SHASUMS256.txt"
  if [[ $platform == linux ]]; then
    archive="node-v${version}-linux-x64.tar.xz"
    node="$root/node-v${version}-linux-x64/bin/node"
    launcher="target/$target/$profile/v8_killer_launcher"
    options=()
  else
    archive=win-x64/node.exe
    node="$root/$archive"
    launcher="target/$target/$profile/v8_killer_launcher.exe"
    options=(--file-output)
    mkdir -p "$root/win-x64"
  fi
  if [[ ! -f $root/$archive ]]; then
    curl --fail --location --retry 3 "https://nodejs.org/dist/v${version}/$archive" -o "$root/$archive"
  fi
  (cd "$root"; grep -F "  $archive" SHASUMS256.txt | sha256sum --check --strict)
  if [[ $platform == linux ]]; then
    tar -xJf "$root/$archive" -C "$root"
  fi
  python scripts/test-node.py "$launcher" "$node" "${options[@]}" --report "reports/$platform-$version-$profile.json"
done
