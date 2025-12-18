
const N = 18

function isprimitive(seq)
    n = length(seq)
    for d in 1:(n-1)
        n % d == 0 || continue
        ok = true
        for i in 1:n
            if seq[i] != seq[(i-1)%d + 1]
                ok = false
                break
            end
        end
        ok && return false
    end
    return true
end

# Check if seq can represent an infinite clock sequence
function isclock(seq)
    p = length(seq)
    pos = 1
    need = 1
    seen = Set{Tuple{Int,Int}}()

    for _ in 1:2000
        rem = need
        while rem > 0
            v = seq[pos]
            v > rem && return false
            rem -= v
            pos += 1
            pos > p && (pos = 1)
        end

        key = (pos, need % (p+50))
        key in seen && return true
        push!(seen, key)

        need += 1
    end

    return false
end

# Count primitive clock-periods of length p
function count_p(p)
    seq = zeros(Int, p)
    cnt = 0

    function dfs(i)
        if i > p
            isprimitive(seq) || return
            isclock(seq) && (cnt += 1)
            return
        end
        for v in 1:10
            seq[i] = v
            dfs(i+1)
        end
    end

    dfs(1)
    return cnt
end

# -------------- Main ----------------

function main()
    C = zeros(Int, N)
    s = 0
    for p in 1:N
        c = count_p(p)
        s += c
        C[p] = s
        println("p=$p  new=$c   C($p)=$s")
    end
    println("\nFinal C(3..N): ", C[3:N])
end

main()
