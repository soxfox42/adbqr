# Changelog

## 1.2.0

- Add "adbqr connect" command to connect to already paired devices.
- Add "adbqr manual" command to connect using a pairing code, instead of via QR code.

## 1.1.0

- Use half-blocks for QR instead of octants.

  Octants are only supported by a handful of fonts, and seem to have some pixel alignment issues in some cases.

- Use low error correction for QR code.
- Use shorter IDs for service name and passcode.

## 1.0.0

- Initial release