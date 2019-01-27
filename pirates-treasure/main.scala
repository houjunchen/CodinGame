// https://www.codingame.com/ide/puzzle/pirates-treasure
import math._
import scala.util._
import Array._

object Solution extends App {
    val w = readInt
    val h = readInt
    var map = new Array[String](h)
    for(i <- 0 until h) {
        map(i) = readLine.replaceAll(" ", "")
    }

    var done = false
    var i = 0
    while(i < h && !done) {
        if(map(i).startsWith("01")) {
            if((i == 0 && map(i+1).startsWith("11")) ||
            (i == h-1 && map(i-1).startsWith("11")) ||
            (i > 0 && i < h-1 && map(i-1).startsWith("11") && map(i+1).startsWith("11"))) {
                println("0 " + i)
                done = true
            }
        }
        if(map(i).endsWith("10")) {
            if((i == 0 && map(i+1).endsWith("11")) ||
            (i == h-1 && map(i-1).endsWith("11")) ||
            (i > 0 && i < h-1 && map(i-1).endsWith("11") && map(i+1).endsWith("11"))) {
                println(w -1 + " " + i)
                done = true
            }
        }
        var j = map(i).indexOf("101")
        if(j >= 0) {
            if((i == 0 && map(i+1).slice(j, j+3) == "111") ||
            (i == h-1 && map(i-1).slice(j, j+3) == "111") ||
            (i > 0 && i < h-1 && map(i-1).slice(j, j+3) == "111" && map(i+1).slice(j, j+3) == "111")) {
                println(j + 1 + " " + i)
                done = true
            }
        }
        i += 1
    }
}
