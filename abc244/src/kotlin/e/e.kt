fun readInts(): List<Int> {
    return readLine()!!.split(" ").map(String::toInt)
}

const val MOD = 998244353L

fun main() {
    val params = readInts()
    val n = params[0]
    val m = params[1]
    val k = params[2]
    val s = params[3] - 1
    val t = params[4] - 1
    val x = params[5] - 1
    val adj = List(n) { mutableListOf<Int>() }
    for (i in 0 until m) {
        val (u, v) = readInts()
        adj[u - 1].add(v - 1)
        adj[v - 1].add(u - 1)
    }

    var dp = List(2) { MutableList<Long>(n) { 0 } }
    dp[0][s] = 1

    for (i in 0 until k) {
        var ndp = List(2) { MutableList<Long>(n) { 0 } }
        for (u in 0 until n) {
            for (v in adj[u]) {
                if (v == x) {
                    ndp[0][v] += dp[1][u]
                    ndp[0][v] %= MOD
                    ndp[1][v] += dp[0][u]
                    ndp[1][v] %= MOD
                } else {
                    ndp[0][v] += dp[0][u]
                    ndp[0][v] %= MOD
                    ndp[1][v] += dp[1][u]
                    ndp[1][v] %= MOD
                }
            }
        }
        dp = ndp
    }

    println(dp[0][t])
}
