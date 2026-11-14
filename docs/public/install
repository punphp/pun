#!/bin/sh
# Installs Pun from its GitHub release:
#
#   curl --proto '=https' --tlsv1.2 -sSf https://github.com/punphp/pun/releases/download/v<version>/install.sh | sh
#
# This file is the template. The release workflow stamps the release's version and the SHA-256 of each archive into
# it (scripts/stamp-installer.py) and publishes the result as that release's install.sh, so an installer only ever
# installs the archives it was built with. Everything runs inside main(), called on the last line: a download cut
# short defines a partial function or none, and runs nothing.
#
# Environment:
#   PUN_INSTALL              install root; the binary goes in $PUN_INSTALL/bin (default: ~/.pun)
#   PUN_REQUIRE_ATTESTATION  1 to fail unless the GitHub build attestation verifies (needs a signed-in gh)

set -eu

PUN_RELEASE='@PUN_VERSION@'
PUN_REPO='punphp/pun'

archive_sha256() {
    case "$1" in
        aarch64-apple-darwin) echo '@SHA256_aarch64-apple-darwin@' ;;
        x86_64-apple-darwin) echo '@SHA256_x86_64-apple-darwin@' ;;
        aarch64-unknown-linux-musl) echo '@SHA256_aarch64-unknown-linux-musl@' ;;
        x86_64-unknown-linux-musl) echo '@SHA256_x86_64-unknown-linux-musl@' ;;
        *) return 1 ;;
    esac
}

say() {
    printf 'pun-installer: %s\n' "$1" >&2
}

fail() {
    say "error: $1"
    exit 1
}

need() {
    command -v "$1" >/dev/null 2>&1 || fail "$1 is required"
}

detect_target() {
    os=$(uname -s)
    arch=$(uname -m)
    case "$arch" in
        x86_64 | amd64) arch=x86_64 ;;
        arm64 | aarch64) arch=aarch64 ;;
        *) fail "unsupported CPU architecture $arch (Pun ships x86_64 and aarch64 binaries)" ;;
    esac
    case "$os" in
        Darwin)
            # A shell running under Rosetta reports x86_64 on Apple silicon; install the native binary.
            if [ "$arch" = x86_64 ] && [ "$(sysctl -n sysctl.proc_translated 2>/dev/null || true)" = 1 ]; then
                arch=aarch64
            fi
            echo "$arch-apple-darwin"
            ;;
        # Static musl binaries, so one build runs on glibc and Alpine alike.
        Linux) echo "$arch-unknown-linux-musl" ;;
        *) fail "unsupported OS $os (Pun ships Linux and macOS binaries)" ;;
    esac
}

# HTTPS only, TLS 1.2 or newer, for the first request and every redirect, and an HTTP error is a failure.
download() {
    if command -v curl >/dev/null 2>&1; then
        curl --proto '=https' --tlsv1.2 --fail --location --silent --show-error --retry 3 --output "$2" "$1" ||
            fail "download failed: $1"
    elif command -v wget >/dev/null 2>&1; then
        wget --https-only --secure-protocol=TLSv1_2 --quiet --output-document="$2" "$1" ||
            fail "download failed: $1 (BusyBox wget cannot enforce HTTPS; install curl)"
    else
        fail "curl or wget is required"
    fi
}

sha256_of() {
    if command -v sha256sum >/dev/null 2>&1; then
        sha256sum "$1" | cut -d ' ' -f 1
    elif command -v shasum >/dev/null 2>&1; then
        shasum -a 256 "$1" | cut -d ' ' -f 1
    elif command -v openssl >/dev/null 2>&1; then
        openssl dgst -sha256 -r "$1" | cut -d ' ' -f 1
    else
        fail "sha256sum, shasum or openssl is required to verify the download"
    fi
}

# GitHub's build attestation proves the archive came from Pun's release workflow on GitHub Actions, which the
# checksum alone cannot: the checksum shares the release with the archive. It needs gh signed in, so without one it
# is skipped unless PUN_REQUIRE_ATTESTATION=1.
verify_attestation() {
    if command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1; then
        gh attestation verify "$1" --repo "$PUN_REPO" \
            --signer-workflow "github.com/$PUN_REPO/.github/workflows/release.yml" \
            --source-ref "refs/tags/v$PUN_RELEASE" --deny-self-hosted-runners >/dev/null ||
            fail "the build attestation of $2 does not verify; the download may have been tampered with"
        say "verified the GitHub build attestation"
    elif [ "${PUN_REQUIRE_ATTESTATION:-0}" = 1 ]; then
        fail "PUN_REQUIRE_ATTESTATION=1 needs the GitHub CLI (gh), signed in"
    else
        say "skipped the build attestation (needs a signed-in gh); the checksum is verified"
    fi
}

main() {
    case "$PUN_RELEASE" in
        @*) fail "this is the installer template; run the install.sh of a release (https://github.com/$PUN_REPO/releases)" ;;
    esac
    need uname
    need tar
    need mktemp

    target=$(detect_target)
    expected=$(archive_sha256 "$target") || fail "no $target build in this release"
    root=${PUN_INSTALL:-}
    if [ -z "$root" ]; then
        [ -n "${HOME:-}" ] || fail "HOME is not set; set PUN_INSTALL"
        root="$HOME/.pun"
    fi
    bin="$root/bin"

    work=$(mktemp -d) || fail "could not create a temporary directory"
    trap 'rm -rf "$work"' EXIT
    trap 'exit 1' HUP INT TERM

    archive="punphp-$target.tar.gz"
    url="https://github.com/$PUN_REPO/releases/download/v$PUN_RELEASE/$archive"
    say "downloading Pun $PUN_RELEASE for $target"
    download "$url" "$work/$archive"

    actual=$(sha256_of "$work/$archive")
    [ "$actual" = "$expected" ] ||
        fail "checksum mismatch for $archive (expected $expected, got $actual); the download may have been tampered with"
    say "verified the SHA-256 checksum"
    verify_attestation "$work/$archive" "$archive"

    # Only regular files and directories under punphp-<target>/: no links, devices, absolute paths or "..".
    tar -tvzf "$work/$archive" >"$work/listing" || fail "$archive is not a gzip tar archive"
    while IFS= read -r line; do
        case "$line" in
            -* | d*) ;;
            *) fail "$archive holds an entry that is not a file or directory: $line" ;;
        esac
    done <"$work/listing"
    tar -tzf "$work/$archive" >"$work/names"
    while IFS= read -r name; do
        case "$name" in
            "punphp-$target/" | "punphp-$target/"[!/]*) ;;
            *) fail "$archive holds an unexpected path: $name" ;;
        esac
        case "/$name/" in
            */../* | */./*) fail "$archive holds an unexpected path: $name" ;;
        esac
    done <"$work/names"
    mkdir "$work/x"
    tar -xzf "$work/$archive" -C "$work/x" || fail "could not extract $archive"
    new="$work/x/punphp-$target/pun"
    [ -f "$new" ] && [ ! -L "$new" ] || fail "$archive holds no pun binary"
    chmod 755 "$new"

    reported=$("$new" --version 2>/dev/null) || fail "the downloaded pun does not run on this machine"
    [ "$reported" = "pun $PUN_RELEASE" ] || fail "the downloaded pun reports '$reported', not 'pun $PUN_RELEASE'"

    mkdir -p "$bin" || fail "could not create $bin"
    # Copied next to the destination first, so the final rename stays on one filesystem: a running `pun` or a
    # second install never sees half a file.
    cp "$new" "$bin/.pun.$$" || fail "could not write to $bin"
    mv -f "$bin/.pun.$$" "$bin/pun" || fail "could not write $bin/pun"
    say "installed pun $PUN_RELEASE to $bin/pun"

    case ":${PATH:-}:" in
        *":$bin:"*) ;;
        *)
            say "$bin is not on your PATH. Add it with:"
            # shellcheck disable=SC2016 # $PATH is printed for the user's shell to expand
            case "$(basename "${SHELL:-sh}")" in
                fish) printf '\n  fish_add_path %s\n\n' "$bin" >&2 ;;
                zsh) printf '\n  echo '\''export PATH="%s:$PATH"'\'' >> ~/.zshrc\n\n' "$bin" >&2 ;;
                bash) printf '\n  echo '\''export PATH="%s:$PATH"'\'' >> ~/.bashrc\n\n' "$bin" >&2 ;;
                *) printf '\n  export PATH="%s:$PATH"\n\n' "$bin" >&2 ;;
            esac
            ;;
    esac
}

main "$@"
