#!/usr/bin/env python3
"""Stamps install.sh with a release's version and the SHA-256 of each archive it may install.

    scripts/stamp-installer.py <version> <dist-dir> <template> > install.sh

<dist-dir> holds the release's punphp-<target>.tar.gz archives. The release workflow runs this
(.github/workflows/installer.yml), so the published installer pins the archives built in the same run.
"""

import hashlib
import pathlib
import re
import sys

TARGETS = [
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "aarch64-unknown-linux-musl",
    "x86_64-unknown-linux-musl",
]


def main() -> None:
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    version, dist, template = sys.argv[1], pathlib.Path(sys.argv[2]), pathlib.Path(sys.argv[3])
    if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.]+)?", version):
        sys.exit(f"stamp-installer: not a release version: {version!r}")

    script = template.read_text().replace("@PUN_VERSION@", version)
    for target in TARGETS:
        archive = dist / f"punphp-{target}.tar.gz"
        if not archive.is_file():
            sys.exit(f"stamp-installer: missing {archive}")
        script = script.replace(f"@SHA256_{target}@", hashlib.sha256(archive.read_bytes()).hexdigest())

    left = re.findall(r"@[A-Z0-9_a-z.-]+@", script)
    if left:
        sys.exit(f"stamp-installer: placeholders left in {template}: {', '.join(sorted(set(left)))}")
    sys.stdout.write(script)


if __name__ == "__main__":
    main()
