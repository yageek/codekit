#!/bin/sh
set -x
export FRAMEWORK_NAME=CodeKit
cd bindings/CodeKit

# Build for iOS
xcodebuild archive \
    -project CodeKit.xcodeproj \
    -scheme ${FRAMEWORK_NAME}-iOS \
    -destination "generic/platform=iOS" \
    -archivePath "archives/${FRAMEWORK_NAME}-iOS"

xcodebuild archive \
    -project CodeKit.xcodeproj \
    -scheme ${FRAMEWORK_NAME}-iOS \
    -destination "generic/platform=iOS Simulator" \
    -archivePath "archives/${FRAMEWORK_NAME}-iOS_Simulator"

xcodebuild archive \
    -project CodeKit.xcodeproj \
    -scheme ${FRAMEWORK_NAME}-macOS \
    -destination "generic/platform=macOS" \
    -archivePath "archives/${FRAMEWORK_NAME}-macOS"

xcodebuild -create-xcframework \
    -archive archives/${FRAMEWORK_NAME}-iOS.xcarchive -framework ${FRAMEWORK_NAME}.framework \
    -archive archives/${FRAMEWORK_NAME}-iOS_Simulator.xcarchive -framework ${FRAMEWORK_NAME}.framework \
    -archive archives/${FRAMEWORK_NAME}-macOS.xcarchive -framework ${FRAMEWORK_NAME}.framework \
    -output xcframeworks/${FRAMEWORK_NAME}.xcframework
