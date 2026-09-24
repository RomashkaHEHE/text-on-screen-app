# Release and deployment

The `Build and publish release` workflow accepts a `vX.Y.Z` tag or a manual `version` input. It builds the signed Tauri installer, stores it as a GitHub artifact, and uploads it to the server when these secrets are configured:

- `RELEASE_SSH_HOST`
- `RELEASE_SSH_USER`
- `RELEASE_SSH_KEY`
- `RELEASE_SSH_PATH` (directory containing the server's `releases` directory)
- `TAURI_SIGNING_PRIVATE_KEY` / `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` when updater signing is enabled

The server always treats `releases/latest.json` as the current version. A deployment can therefore be rolled forward or back by replacing that one file and keeping immutable version directories.
