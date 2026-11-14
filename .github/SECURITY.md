# Security policy

## Report a vulnerability

Do not open a public issue for a security problem. Use **Report a vulnerability** on the repository's
[Security tab](https://github.com/punphp/pun/security/advisories/new). You get a reply within seven days.

Only the latest release receives security fixes.

## What Pun runs

Pun reads PHP files and never executes them, with two exceptions. Run these only on code you trust, or inside a
sandbox:

- **Blade formatting** (`pun format --blade`, or the `laravel/blade` rule in `pun.toml`) starts Node and loads
  Prettier and its plugins from the project's `node_modules`.
- **`pun migrate`** runs PHP to read a `.php-cs-fixer.php` or `rector.php`. Both files are PHP code, and reading
  them loads the project's `vendor/autoload.php`.

`pun.toml` comes with the repository. It can turn rules on and choose which files `--fix` rewrites, but only inside
the project: `[files] paths` cannot point outside it.

## The Composer package

`vendor/bin/pun` downloads the binary for your platform from this repository's GitHub release and checks its
SHA-256 checksum, which is published in the same release. The checksum detects a corrupt or incomplete download.

## The standalone installer

Each release's `install.sh` holds the SHA-256 of every archive in that release, written by the release workflow,
and refuses any other archive. Pun's release workflow attests the archives and `install.sh` with GitHub artifact
attestations (Sigstore build provenance). With a signed-in GitHub CLI, the installer checks the archive's
attestation against `punphp/pun`'s `release.yml` and the release tag. Verify the installer itself with
`gh attestation verify install.sh --repo punphp/pun`.
