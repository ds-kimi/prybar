#!/bin/sh
# Signs and notarizes the macOS release files inside CI only.
# Reads secrets from the environment and never prints them; no `set -x`.
# Usage: macos-sign.sh <dir with prybar, prybar-mcp, Prybar.app>
set -eu
dir="$1"
here="$(cd "$(dirname "$0")/../.." && pwd)"
tmp="${RUNNER_TEMP:?}/prybar-sign"
umask 077
mkdir -p "$tmp"
keychain="$tmp/sign.keychain-db"
kc_pass="$(openssl rand -hex 32)"

# Decoded files go to private temp files. `security` only takes passwords as
# arguments; that is accepted because this job runs no third-party code.
printf '%s' "$MACOS_CERT_P12_BASE64" | base64 --decode > "$tmp/cert.p12"
printf '%s' "$APPLE_API_KEY_P8_BASE64" | base64 --decode > "$tmp/api_key.p8"

security create-keychain -p "$kc_pass" "$keychain"
security set-keychain-settings -lut 3600 "$keychain"
security unlock-keychain -p "$kc_pass" "$keychain"
security import "$tmp/cert.p12" -k "$keychain" -P "$MACOS_CERT_PASSWORD" -T /usr/bin/codesign >/dev/null
security set-key-partition-list -S apple-tool:,apple: -s -k "$kc_pass" "$keychain" >/dev/null
# Word splitting is wanted: the existing search list is one path per word.
# shellcheck disable=SC2046
security list-keychains -d user -s "$keychain" $(security list-keychains -d user | tr -d '"')
rm -f "$tmp/cert.p12"

sign() {
  codesign --force --options runtime --timestamp --keychain "$keychain" \
    --sign "$MACOS_SIGN_IDENTITY" "$@"
}
sign "$dir/prybar"
sign "$dir/prybar-mcp"
sign --entitlements "$here/crates/prybar-gui/bundle/entitlements.plist" "$dir/Prybar.app"
for f in "$dir/prybar" "$dir/prybar-mcp" "$dir/Prybar.app"; do
  codesign --verify --strict --verbose=1 "$f"
done

# notarytool takes a zip; command line tools are notarized inside it.
ditto -c -k --keepParent "$dir" "$tmp/notarize.zip"
key_args="--key $tmp/api_key.p8 --key-id $APPLE_API_KEY_ID --issuer $APPLE_API_ISSUER_ID"
# Word splitting of key_args is wanted; none of its parts contain spaces.
# shellcheck disable=SC2086
out="$(xcrun notarytool submit "$tmp/notarize.zip" --wait --timeout 30m $key_args)" || status=$?
echo "$out"
if [ "${status:-0}" != 0 ] || ! printf '%s' "$out" | grep -q 'status: Accepted'; then
  id="$(printf '%s' "$out" | awk '/^ *id:/ { print $2; exit }')"
  # shellcheck disable=SC2086
  [ -n "$id" ] && xcrun notarytool log "$id" $key_args
  exit 1
fi
rm -f "$tmp/notarize.zip"
xcrun stapler staple "$dir/Prybar.app"
spctl -a -vv -t exec "$dir/Prybar.app"
