import Foundation

let args = CommandLine.arguments
let exitAfter: Double? = args.firstIndex(of: "--exit-after").flatMap { $0 + 1 < args.count ? Double(args[$0 + 1]) : nil }
let selfTest = args.contains("--selftest")
let testMode = selfTest || exitAfter != nil

let keycodeK: Int64 = 40
let keycodeEsc: Int64 = 53
let exitHolds: [Int64: Double] = [keycodeK: 2, keycodeEsc: 5]
