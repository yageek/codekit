// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "CodeKit",
    platforms: [
        .macOS(.v10_13), .iOS(.v11)
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "CodeKit",
            targets: ["CodeKit"])
    ],
    dependencies: [],
    targets: [
        .binaryTarget(
            name: "CodeKit",
            url: "https://github.com/yageek/codekit/releases/download/v1.0.0/CodeKit-1.0.0.zip",
            checksum: "42199d5cecfc699fc1c36f384558704cdfcf39f030d1b5f37f5550e15cc686cf"
        )
    ]
)