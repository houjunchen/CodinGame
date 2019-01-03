// https://www.codingame.com/ide/puzzle/ghost-legs
import Foundation

let inputs = (readLine()!).characters.split{$0 == " "}.map(String.init)
let W = Int(inputs[0])!, H = Int(inputs[1])!
let count = (W+2)/3
var indexes = Array(0 ..< count)

let starts = readLine()!.components(separatedBy: "  ")
for i in 0...(H-3) {
    let line = readLine()!
    for j in 0..<count-1 {
        if line[line.index(line.startIndex, offsetBy: j*3)...line.index(line.startIndex, offsetBy: j*3+1)]=="|-" {
            indexes.swapAt(j, j+1)
        }
    }
}
let ends = readLine()!.components(separatedBy: "  ")

var mapping: [String: String] = [:]
for (i, v) in indexes.enumerated() {
    mapping[starts[v]] = ends[i]
}
for s in starts {
    print(s+mapping[s]!)
}
