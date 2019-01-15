// https://www.codingame.com/ide/puzzle/dead-mens-shot
import Foundation

struct Point {
    var x: Int
    var y: Int
}

func cross(p0: Point, p1: Point, p2: Point) -> Bool {
    return (p1.x-p0.x)*(p2.y-p0.y) - (p1.y-p0.y)*(p2.x-p0.x) >= 0
}

func isInPolygon(p: Point, points: [Point]) {
    for i in 0..<points.count {
        let j = (i == points.count - 1 ? 0 : i + 1)
        if cross(p0: p, p1: points[i], p2: points[j]) == false {
            print("miss")
            return
        }
    }
    print("hit")
}

var points: [Point] = []

let N = Int(readLine()!)!
if N > 0 {
    for i in 0...(N-1) {
        let inputs = (readLine()!).characters.split{$0 == " "}.map(String.init)
        points.append(Point(x: Int(inputs[0])!, y: Int(inputs[1])!))
    }
}

let M = Int(readLine()!)!
if M > 0 {
    for i in 0...(M-1) {
        let inputs = (readLine()!).characters.split{$0 == " "}.map(String.init)
        isInPolygon(p: Point(x: Int(inputs[0])!, y: Int(inputs[1])!), points: points)
    }
}
