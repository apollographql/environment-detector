# Release Checklist

A standard release, from `main` (i.e. 0.1.0, v0.2.3, v4.5.6, etc.)

_N.B All the version numbers above follow Semantic Versioning, and specifically its notion of encoding pre-release information after the `-` for more information see the [Semantic Versioning specification](https://semver.org/)_

## Standard Release

As noted above, a standard release takes the form of a tagged commit on the `main` branch, and includes all
commits previous to it. Preparing it involves several phases:

### Tag and build release

This part of the release process is handled by CircleCI. When a version tag is pushed, a workflow will be triggered, and is reponsible for building the library and publishing a new verison to crates.io.

1. Create a PR that bumps the package version inside `Cargo.toml`.
2. Have your PR merged to `main`.
3. Once merged, run `git checkout main` and `git pull`.
4. Sync your local tags with the remote tags by running `git tag -d $(git tag) && git fetch --tags`.
5. Tag the commit by running either `git tag -a #.#.# -m "#.#.#"`.
6. Run `git push --tags`.
7. Wait for CI to pass.
8. Create a new [release](https://github.com/apollographql/environment-detector/releases) from the associated tag.
9. Click `Edit`, paste the release notes from the changelog, and save the changes to the release.
