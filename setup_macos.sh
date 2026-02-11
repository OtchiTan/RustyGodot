#!/usr/bin/env bash

# Clear any 'Quarantine' or partial signature mess
xattr -cr .godot_bin/Godot.app
rm -rf .godot_bin/Godot.app/_CodeSignature

# Create the debug permissions file
cat <<EOF > debug.plist
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.get-task-allow</key>
    <true/>
    <key>com.apple.security.cs.disable-library-validation</key>
    <true/>
</dict>
</plist>
EOF

# Force sign the bundle (ignoring the "unsealed contents" error)
codesign --force --deep --sign - --entitlements debug.plist .godot_bin/Godot.app

# lean up
rm debug.plist

# Verify it worked (should say 'valid on disk')
codesign -v .godot_bin/Godot.app