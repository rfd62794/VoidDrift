# GitHub Actions Workflows

This directory contains CI/CD workflows for VoidDrift.

## Required GitHub Secrets

### deploy-telemetry.yml
- `DEPLOY_KEY` - SSH private key for server deployment (deploy key with write access to server only)
- `SSH_HOST` - Server hostname (e.g., rfditservices.com)
- `SSH_USER` - SSH username for deployment
- `SSH_PORT` - SSH port (default: 22)

### deploy-wasm.yml
- `BUTLER_CREDENTIALS` - Butler API credentials for itch.io deployment

**How to get Butler credentials:**
Butler stores credentials locally after running `butler login`. The credentials file is located at:
- Windows: `C:\Users\<username>\.config\itch\butler_credentials.json`
- Linux/macOS: `~/.config/itch/butler_credentials.json`

Copy the contents of this file and add it as the `BUTLER_CREDENTIALS` secret in GitHub repository settings.
