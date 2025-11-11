# GitHub Repository Setup Guide

This guide walks you through setting up your GitHub repository with proper metadata and settings.

## 1. Repository Description & Topics

Navigate to your repository settings and add:

**Description:**
```
A simple, fast, and reliable Rust CLI tool for managing port mappings
```

**Website:**
```
https://github.com/mgorunuch/ports-manager
```

**Topics (click "Add topics"):**
- rust
- cli
- port-management
- devtools
- developer-tools
- configuration-management
- dotfiles
- command-line-tool
- ports
- networking
- rust-cli
- cargo
- clap

## 2. Repository Features

In Settings → General → Features, enable:
- ✅ Issues
- ✅ Discussions (recommended)
- ✅ Projects (optional)
- ✅ Preserve this repository (optional)
- ✅ Sponsorships (if applicable)

## 3. Branch Protection Rules

Navigate to Settings → Branches → Add rule

**Branch name pattern:** `main`

Enable the following:
- ✅ Require a pull request before merging
  - ✅ Require approvals (1 approval recommended)
  - ✅ Dismiss stale pull request approvals when new commits are pushed
  - ✅ Require review from Code Owners
- ✅ Require status checks to pass before merging
  - ✅ Require branches to be up to date before merging
  - Add required status checks:
    - `test (ubuntu-latest, stable)`
    - `test (macos-latest, stable)`
    - `test (windows-latest, stable)`
    - `lint`
    - `build (ubuntu-latest)`
- ✅ Require conversation resolution before merging
- ✅ Require signed commits (recommended)
- ✅ Include administrators

## 4. Security Settings

Navigate to Settings → Security & analysis

- ✅ Dependency graph
- ✅ Dependabot alerts
- ✅ Dependabot security updates
- ✅ Grouped security updates
- ✅ Secret scanning (if available)
- ✅ Push protection (if available)

## 5. Actions Settings

Navigate to Settings → Actions → General

**Actions permissions:**
- ✅ Allow all actions and reusable workflows

**Workflow permissions:**
- ✅ Read and write permissions
- ✅ Allow GitHub Actions to create and approve pull requests

## 6. Secrets Configuration (for releases)

Navigate to Settings → Secrets and variables → Actions

Add the following secrets:

1. **CODECOV_TOKEN** (optional, for code coverage)
   - Sign up at https://codecov.io
   - Add your repository
   - Copy the token and add it here

2. **CARGO_REGISTRY_TOKEN** (for publishing to crates.io)
   - Go to https://crates.io/settings/tokens
   - Create a new token
   - Add it here

## 7. GitHub Pages (optional)

If you want to host documentation:

Navigate to Settings → Pages
- Source: Deploy from a branch
- Branch: `gh-pages` / `root`

## 8. Labels

Create these labels for better organization:

| Label | Description | Color |
|-------|-------------|-------|
| `bug` | Something isn't working | `#d73a4a` |
| `enhancement` | New feature or request | `#a2eeef` |
| `documentation` | Improvements or additions to documentation | `#0075ca` |
| `good first issue` | Good for newcomers | `#7057ff` |
| `help wanted` | Extra attention is needed | `#008672` |
| `dependencies` | Pull requests that update a dependency file | `#0366d6` |
| `rust` | Related to Rust code | `#dea584` |
| `github-actions` | Related to GitHub Actions workflows | `#000000` |
| `performance` | Performance improvements | `#1d76db` |
| `security` | Security-related issues | `#ee0701` |

## 9. Verify GitHub Actions

After pushing your changes:

1. Go to the Actions tab
2. Verify that workflows are running
3. Check that all jobs complete successfully

## 10. Create First Release (optional)

When ready to release:

```bash
# Tag your release
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

This will trigger the release workflow and create GitHub releases with binaries.

## 11. Publish to crates.io (optional)

```bash
# Login to crates.io
cargo login

# Publish (make sure CARGO_REGISTRY_TOKEN is set in GitHub secrets)
cargo publish --dry-run  # Test first
cargo publish
```

## Checklist

- [ ] Description and topics added
- [ ] Features enabled (Issues, Discussions, etc.)
- [ ] Branch protection rules configured
- [ ] Security settings enabled
- [ ] Secrets configured (if needed)
- [ ] Labels created
- [ ] GitHub Actions verified
- [ ] First release created (optional)
- [ ] Published to crates.io (optional)
