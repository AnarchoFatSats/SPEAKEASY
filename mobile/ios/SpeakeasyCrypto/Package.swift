// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "SpeakeasyCrypto",
    platforms: [
        .iOS(.v13),
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "SpeakeasyCrypto",
            targets: ["SpeakeasyCrypto"]),
    ],
    dependencies: [
        .package(url: "https://github.com/signalapp/libsignal-client.git", from: "0.22.0")
    ],
    targets: [
        .target(
            name: "SpeakeasyCrypto",
            dependencies: [
                .product(name: "SignalClient", package: "libsignal-client")
            ],
            path: ".",
            exclude: ["info.plist"] // Exclude non-source files if any
        ),
        .testTarget(
            name: "SpeakeasyCryptoTests",
            dependencies: ["SpeakeasyCrypto"],
            path: "Tests"
        ),
    ]
)
