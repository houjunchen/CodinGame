// https://www.codingame.com/ide/puzzle/111-rubiks-cube-movements
import math._
import scala.util._

object Solution extends App {
    val inputs = readLine.split(" ")
    var face1 = readLine
    var face2 = readLine

    val rotation = Map(
        "x" -> Map("R" -> "R", "L" -> "L", "F" -> "U", "U" -> "B", "B" -> "D", "D" -> "F"),
        "x'" -> Map("R" -> "R", "L" -> "L", "F" -> "D", "D" -> "B", "B" -> "U", "U" -> "F"),
        "y" -> Map("U" -> "U", "D" -> "D", "F" -> "L", "L" -> "B", "B" -> "R", "R" -> "F"),
        "y'" -> Map("U" -> "U", "D" -> "D", "F" -> "R", "R" -> "B", "B" -> "L", "L" -> "F"),
        "z" -> Map("F" -> "F", "B" -> "B", "U" -> "R", "R" -> "D", "D" -> "L", "L" -> "U"),
        "z'" -> Map("F" -> "F", "B" -> "B", "U" -> "L", "L" -> "D", "D" -> "R", "R" -> "U")
    )

    for(i <- inputs) {
        face1 = rotation(i)(face1)
        face2 = rotation(i)(face2)
    }
    printf("%s\n%s", face1, face2)
}
