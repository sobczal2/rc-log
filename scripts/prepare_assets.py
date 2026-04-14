#!/usr/bin/env python3
"""
prepare_assets.py — Encode all maneuver variation videos into 3 sizes.

For each variation directory found under content/maneuver/, this script:
  - Expects a source file named `video.mp4` inside the variation directory.
    - Reads `variation.json` to obtain the `videoAssetId`.
  - Encodes the source video into three outputs:
            assets/videos/{videoAssetId}_small.mp4   (480p, 1 Mbps)
            assets/videos/{videoAssetId}_medium.mp4  (720p, 2.5 Mbps)
            assets/videos/{videoAssetId}_large.mp4   (1080p, 5 Mbps)
  - Skips encoding for any output that is already newer than the source
    (unless --force is passed).

Usage:
  python scripts/prepare_assets.py [--force]

Requirements:
  - ffmpeg must be available on PATH.
  - Run from the repository root.
"""

import argparse
import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
CONTENT_DIR = REPO_ROOT / "content" / "maneuver"
OUTPUT_DIR = REPO_ROOT / "assets" / "videos"

SIZES = [
    ("small",  "scale=-2:480",  "1M"),
    ("medium", "scale=-2:720",  "2.5M"),
    ("large",  "scale=-2:1080", "5M"),
]


def check_ffmpeg() -> None:
    try:
        subprocess.run(
            ["ffmpeg", "-version"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=True,
        )
    except FileNotFoundError:
        print("ERROR: ffmpeg not found on PATH. Please install ffmpeg.", file=sys.stderr)
        sys.exit(1)


def is_up_to_date(source: Path, output: Path) -> bool:
    """Return True if output exists and is newer than source."""
    return output.exists() and output.stat().st_mtime >= source.stat().st_mtime


def encode(source: Path, output: Path, scale: str, bitrate: str) -> bool:
    """
    Run ffmpeg to encode source → output at the given scale and bitrate.
    Returns True on success, False on failure.
    """
    cmd = [
        "ffmpeg",
        "-i", str(source),
        "-vf", scale,
        "-c:v", "libx264",
        "-preset", "fast",
        "-b:v", bitrate,
        "-c:a", "aac",
        "-movflags", "+faststart",
        "-y",
        str(output),
    ]
    print(f"  Encoding → {output.relative_to(REPO_ROOT)}  [{scale}, {bitrate}]")
    result = subprocess.run(cmd, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        print(f"    ERROR: ffmpeg failed:\n{result.stderr}", file=sys.stderr)
        return False
    return True


def process_variation(variation_dir: Path, force: bool) -> None:
    source = variation_dir / "video.mp4"
    variation_json = variation_dir / "variation.json"

    if not variation_json.exists():
        print(f"  SKIP: no variation.json in {variation_dir.relative_to(REPO_ROOT)}")
        return

    with open(variation_json, encoding="utf-8") as f:
        data = json.load(f)

    asset_id = data.get("videoAssetId")
    if not asset_id:
        print(
            f"  SKIP: 'videoAssetId' missing in "
            f"{variation_json.relative_to(REPO_ROOT)}"
        )
        return

    if not source.exists():
        print(
            f"  SKIP: no video.mp4 in {variation_dir.relative_to(REPO_ROOT)} "
            f"(asset: {asset_id})"
        )
        return

    any_encoded = False
    for size_label, scale, bitrate in SIZES:
        output = OUTPUT_DIR / f"{asset_id}_{size_label}.mp4"
        if not force and is_up_to_date(source, output):
            print(f"  UP-TO-DATE: {output.relative_to(REPO_ROOT)}")
            continue
        success = encode(source, output, scale, bitrate)
        if not success:
            print(
                f"  WARN: encoding failed for {output.name}, continuing.",
                file=sys.stderr,
            )
        else:
            any_encoded = True

    if not any_encoded:
        pass  # all sizes were up-to-date or already reported


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Encode maneuver variation videos into 3 sizes."
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Re-encode outputs even when they are up-to-date.",
    )
    args = parser.parse_args()

    check_ffmpeg()
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    if not CONTENT_DIR.exists():
        print(f"ERROR: content directory not found: {CONTENT_DIR}", file=sys.stderr)
        sys.exit(1)

    processed = 0
    for maneuver_dir in sorted(CONTENT_DIR.iterdir()):
        if not maneuver_dir.is_dir():
            continue
        for variation_dir in sorted(maneuver_dir.iterdir()):
            if not variation_dir.is_dir():
                continue
            label = (
                f"{maneuver_dir.name}/{variation_dir.name}"
            )
            print(f"\n[{label}]")
            process_variation(variation_dir, force=args.force)
            processed += 1

    print(f"\nDone. Checked {processed} variation director(ies).")
    print(f"Output directory: {OUTPUT_DIR.relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
