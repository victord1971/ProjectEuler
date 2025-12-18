##############################################################
#   Project Euler 908 — Brute Force Period Search (Julia)
#   ВЕРСІЯ БЕЗ ПОМИЛОК, БЕЗПЕЧНА ДЛЯ THREADS
##############################################################

using Printf
using Base.Threads


############ ПАРАМЕТРИ — МІНЯЙ ТУТ ############
const N = 16                 # максимум p (довжина періоду)
const MAX_ELEM = 6           # елементи блоку: 1..MAX_ELEM
const MAX_SEGMENTS = 1200    # симуляція сегментів
###############################################


#######################
# Перевірка періодичності
#######################
function is_clock_periodic(seq::Vector{Int}; max_segments::Int=MAX_SEGMENTS)
    p = length(seq)
    pos = 1
    cur_sum = 1
    R = p + maximum(seq) + 5
    seen = Dict{Tuple{Int,Int},Int}()

    for seg in 1:max_segments
        rem = cur_sum

        while rem > 0
            v = seq[pos]
            if v > rem
                return false
            end
            rem -= v
            pos += 1
            if pos > p
                pos = 1
            end
        end

        key = (pos, cur_sum % R)
        if haskey(seen, key)
            return true
        end
        seen[key] = seg

        cur_sum += 1
    end

    return false
end


########################
# Перевірка примітивного періоду
########################
function is_primitive_period(seq::Vector{Int})
    p = length(seq)
    for d in 1:(p-1)
        if p % d == 0
            ok = true
            for i in 1:p
                if seq[i] != seq[(i-1) % d + 1]
                    ok = false
                    break
                end
            end
            if ok
                return false
            end
        end
    end
    return true
end


#########################################
# Повний перебір усіх seq довжини p
#########################################
function count_for_length(p::Int; max_elem::Int=MAX_ELEM, max_segments::Int=MAX_SEGMENTS)
    cnt = Ref(0)
    seq = Vector{Int}(undef, p)

    function dfs(pos::Int)
        if pos > p
            if !is_primitive_period(seq)
                return
            end
            if is_clock_periodic(seq; max_segments=max_segments)
                cnt[] += 1
            end
            return
        end

        for v in 1:max_elem
            seq[pos] = v
            dfs(pos + 1)
        end
    end

    dfs(1)
    return cnt[]
end


###############################################
# БЕЗПЕЧНА багатопоточність: threads-safe
###############################################
function compute_C(N::Int; max_elem::Int=MAX_ELEM, max_segments::Int=MAX_SEGMENTS)
    results = Vector{Int}(undef, N)   # кожен p має свою власну комірку

    @threads for p in 1:N
        @printf("[thread %d] p = %d ... ", threadid(), p)
        t0 = time()
        res = count_for_length(p; max_elem=max_elem, max_segments=max_segments)
        results[p] = res      # кожен потік пише тільки у results[p]
        dt = time() - t0
        @printf("found=%d  time=%.2f s\n", res, dt)
    end

    return sum(results)
end


##############
# MAIN
##############
function main()
    println("============================================")
    println("  Project Euler 908 — Brute Force (Julia)")
    println("============================================")
    println("Parameters:")
    println("  N            = $N")
    println("  MAX_ELEM     = $MAX_ELEM")
    println("  MAX_SEGMENTS = $MAX_SEGMENTS")
    println("Threads available: ", nthreads())
    println("--------------------------------------------")

    t0 = time()
    total = compute_C(N; max_elem=MAX_ELEM, max_segments=MAX_SEGMENTS)
    dt = time() - t0

    println("--------------------------------------------")
    println("TOTAL C(N) = ", total)
    @printf("Time: %.2f seconds\n", dt)
    println("--------------------------------------------")
end

main()
