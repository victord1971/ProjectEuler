using Printf

function solve940_fast_debug()
    M = 1123581313

    # ---------- Time helpers ----------
    function now()
        return time()
    end

    function fmt_time(t)
        if t < 60
            return @sprintf("%.1f s", t)
        elseif t < 3600
            return @sprintf("%d min %.0f s", Int(t÷60), t%60)
        else
            return @sprintf("%d h %d min", Int(t÷3600), Int((t%3600)÷60))
        end
    end

    # ---------- Fibonacci ----------
    f = Vector{Int128}(undef, 50)
    f[1] = 1
    f[2] = 1
    for i in 3:50
        f[i] = f[i-1] + f[i-2]
    end

    A = [Int(f[i] ÷ M) for i in 1:50]
    B = [Int(f[i] % M) for i in 1:50]

    # ---------- Small binomial ----------
    function smallbin(n, k)
        k < 0 || k > n && return 0
        k = min(k, n-k)
        r = 1
        for i in 1:k
            r = r * (n - k + i) ÷ i
        end
        return r % M
    end

    # ---------- Big binomial with cache ----------
    cache = Dict{Tuple{Int,Int},Int}()

    function bigbin(n::Int, k::Int)
        if haskey(cache, (n,k))
            return cache[(n,k)]
        end
        if k < 0 || k > n
            cache[(n,k)] = 0
            return 0
        end
        if n >= M
            cache[(n,k)] = 0
            return 0
        end

        kk = min(k, n-k)
        r::Int128 = 1
        for i in 1:kk
            r = (r * (n - kk + i)) % M
            r = (r * powermod(i, M-2, M)) % M
        end
        cache[(n,k)] = Int(r)
        return Int(r)
    end

    # ---------- Progress bar ----------
    total = 49 * 49   # 2401 operations
    done = 0
    t0 = now()

    function print_progress()
        elapsed = now() - t0
        percent = done / total

        eta = percent == 0 ? 0.0 : elapsed * (1/percent - 1)

        bar_len = 20
        filled = Int(round(percent * bar_len))
        bar = "█"^filled * "░"^(bar_len - filled)

        print("\r[$bar]  $(round(percent*100; digits=1))%   elapsed: $(fmt_time(elapsed))   ETA: $(fmt_time(eta))")
        flush(stdout)
    end

    # ---------- Main sum ----------
    S = 0
    for i in 2:50
        for j in 2:50
            done += 1
            if done % 10 == 0 || done == total
                print_progress()
            end

            x = smallbin(A[i] + A[j], A[i])
            y = bigbin(B[i] + B[j], B[i])
            S = (S + x * y) % M
        end
    end

    println()

    S = (S - 49*49) % M
    println("Result S(50) mod M = $S")

    return S
end

solve940_fast_debug()
