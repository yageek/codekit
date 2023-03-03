#!/bin/sh

export FRAMEWORK_NAME=CodeKit
cd bindings/CodeKit

# Build for iOS
xcodebuild archive \
    -project CodeKit.xcodeproj \
    -scheme ${FRAMEWORK_NAME} \
    -destination "generic/platform=iOS" \
    -archivePath "archives/${FRAMEWORK_NAME}-iOS"
