// swift-tools-version:5.3
import PackageDescription

let package = Package(
    name: "RCKCodeKit",
    platforms: [
        .macOS(.v10_13), .iOS(.v11)
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "RCKCodeKit",
            targets: ["RCKCodeKit"])
    ],
    dependencies: [],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "RCKCodeKit"
        ),
        .binaryTarget(
            name: "RCKCodeKit",
            url: "https://github.com/yageek/codekit/releases/download/v1.0.0/CodeKit-1.0.0.zip",
            checksum: "69208562f28b68f966166a1b3127bacde3b28d6e0e138fcd91016f7f12cffa73"
        )
    ]
)