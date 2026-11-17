import Foundation

let logURL = URL(fileURLWithPath: NSHomeDirectory()).appendingPathComponent("Library/Logs/BabyBoop.log")

func resetLog() { try? "".write(to: logURL, atomically: true, encoding: .utf8) }

func log(_ s: String) {
    let line = "\(Date()) \(s)\n"
    if let h = try? FileHandle(forWritingTo: logURL) { h.seekToEndOfFile(); h.write(line.data(using: .utf8)!); h.closeFile() }
}
