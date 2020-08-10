import java.util.*

fun readInt(): Int {
    return readLine()!!.toInt()
}

fun readInts(): List<Int> {
    return readLine()!!.split(" ").map(String::toInt)
}

class Solution {
    fun solve() {
        val n = readInt()
        val red = Array(n + 1) { mutableListOf<Int>() }
        val blue = Array(n + 1) { mutableListOf<Int>() }
        val l = IntArray(n)
        val r = IntArray(n)
        var ans = 0L
        for (i in 0 until n) {
            val (ki, li, ri) = readInts()
            if (li >= ri)
                red[ki].add(i)
            else
                blue[ki].add(i)
            l[i] = li
            r[i] = ri
            ans += li.coerceAtMost(ri)
        }

        val pq = PriorityQueue<Pair<Int, Int>>(compareBy<Pair<Int, Int>> { it.first }.thenBy { it.second })
        for (i in 1 until n + 1) {
            for (j in red[i])
                pq.add(Pair(l[j] - r[j], j))
            while (pq.size > i)
                pq.poll();
        }
        while (pq.isNotEmpty())
            ans += pq.poll().first

        for (i in n downTo 1) {
            for (j in blue[i])
                pq.add(Pair(r[j] - l[j], j))
            while (pq.size > n - i)
                pq.poll();
        }
        while (pq.isNotEmpty())
            ans += pq.poll().first

        println(ans)
    }
}

fun main() {
    val t = readInt()
    for (i in 1 until t + 1) {
        val solution = Solution()
        solution.solve()
    }
}
