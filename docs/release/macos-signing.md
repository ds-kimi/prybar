# macOS signing and notarization

Release builds for macOS are signed with a Developer ID Application
certificate and notarized by Apple, but only when the six repository secrets
below exist. Without them the release workflow prints a notice and ships
unsigned builds, as before.

If only some of the six are set, the release fails instead of shipping
unsigned builds quietly.

Signing happens only in GitHub Actions, only for `v*` tag pushes on
`ds-kimi/prybar`, never on pull requests or forks. It runs in its own job
(`macos`) that downloads the already built files and runs no cargo or other
third-party build code, so nothing untrusted shares the runner with the
secrets. The secrets live in the `release` environment, not at repository
level, and are passed only to the signing step. The certificate lives in a
temporary keychain with a random password that is deleted at the end of the
job. Notarization gives up after 30 minutes and prints Apple's log when a
submission is rejected.

Tags must look like `v1.2.3`: the app bundle version only allows numbers, so
a tag such as `v1.2.0-rc.1` fails the release at its first step, on every
platform.

## What gets signed

Each macOS archive (`prybar-<tag>-<target>.zip`, a zip since signing was
added; it used to be `.tar.gz`) contains:

- `prybar` and `prybar-mcp`, signed with the hardened runtime;
- `Prybar.app` (the desktop interface), signed, notarized and stapled.

Notarization covers all three. Command line tools cannot be stapled, so
Gatekeeper checks them online the first time they run.

## One-time setup (owner)

Run these on your own Mac. Never paste the values into an issue, a commit or a
chat.

### 1. Developer ID Application certificate

1. Open Keychain Access > Certificate Assistant > Request a Certificate From a
   Certificate Authority. Save the request to disk.
2. On <https://developer.apple.com/account/resources/certificates>, create a
   **Developer ID Application** certificate from that request, download it and
   double-click it to add it to your login keychain.
3. In Keychain Access, under My Certificates, right-click the certificate >
   Export, format `.p12`, set a strong password.
4. Note the identity name:

   ```sh
   security find-identity -v -p codesigning
   ```

   Use the full quoted text, for example `Developer ID Application: Name (TEAMID)`.

### 2. App Store Connect API key

1. On <https://appstoreconnect.apple.com/access/integrations/api>, create a team
   key with the **Developer** role.
2. Download the `.p8` file (only possible once). Note the Key ID and the Issuer
   ID shown on the page.

### 3. Protect release tags and the `release` environment

The workflow file comes from the tagged commit, so anyone who can push a `v*`
tag could change it. Limit who can:

1. Settings > Rules > Rulesets > New tag ruleset: target `v*`, restrict
   creations, updates and deletions to yourself (bypass list).
2. Settings > Environments > New environment `release`. Under Deployment
   branches and tags, choose Selected and add the tag pattern `v*`.
   Optionally add yourself as a required reviewer, so every signing run waits
   for your click.

### 4. Environment secrets

From the repo folder, with the GitHub CLI logged in:

```sh
base64 -i DeveloperID.p12 | gh secret set MACOS_CERT_P12_BASE64 --env release
gh secret set MACOS_CERT_PASSWORD --env release       # prompts, paste the p12 password
gh secret set MACOS_SIGN_IDENTITY --env release       # prompts, paste the identity name
base64 -i AuthKey_XXXXXXXXXX.p8 | gh secret set APPLE_API_KEY_P8_BASE64 --env release
gh secret set APPLE_API_KEY_ID --env release          # prompts
gh secret set APPLE_API_ISSUER_ID --env release       # prompts
```

`gh secret set` without a value reads from a hidden prompt, so nothing lands in
your shell history. Then delete the exported `.p12` and keep the `.p8` in a
password manager or delete it.

### 5. Release

Push a tag (`git tag v0.2.0 && git push origin v0.2.0`). In the
`macos` jobs, the step "sign and notarize" should run and end with `spctl`
reporting `source=Notarized Developer ID`.

## Check a download

```sh
codesign --verify --strict --verbose=2 Prybar.app
spctl -a -vv -t exec Prybar.app
xcrun stapler validate Prybar.app
```

## Rotate or revoke

- API key: revoke it in App Store Connect, create a new one, rerun the three
  `APPLE_API_*` commands.
- Certificate: revoke it on the developer site only if it leaked (apps already
  notarized keep working unless Apple revokes them). Create a new one, rerun the
  three `MACOS_*` commands.
- Stop signing: delete all six with `gh secret delete <NAME> --env release`;
  the next release is unsigned (deleting only some makes it fail).

## Files

- `.github/workflows/release.yml`: build, bundle, sign, archive, publish.
- `scripts/release/macos-bundle.sh`: builds `Prybar.app`.
- `scripts/release/macos-secrets-check.sh`: sign, skip, or fail on a partial setup.
- `scripts/release/macos-sign.sh`: keychain, codesign, notarytool, staple.
- `scripts/release/macos-cleanup.sh`: removes the keychain and decoded files.
- `crates/prybar-gui/bundle/`: `Info.plist` template and entitlements.

There is no app icon yet; the bundle uses the system default.
