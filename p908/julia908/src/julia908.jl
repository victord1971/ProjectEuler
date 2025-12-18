############################################################
#   Project Euler 908 — scalable algorithm for C(10000)
#   Standalone Julia code with built-in Mobius function.
############################################################

const MOD = 1_111_211_113
const MAXN = 10_000

############################################################
# μ(n) — Mobius function  (no packages needed)
############################################################

function mobius(n::Int)
    m = n
    cnt = 0
    p = 2

    while p*p <= m
        if m % p == 0
            m ÷= p
            if m % p == 0
                return 0        # square factor
            end
            cnt += 1
        end
        p += 1
    end

    if m > 1
        cnt += 1                # remaining prime
    end

    return (cnt % 2 == 0) ? 1 : -1
end

############################################################
# factorial mod M for all p ≤ N
############################################################

function factorials_mod(n::Int, M::Int)
    f = Vector{Int}(undef, n)
    f[1] = 1
    for i in 2:n
        f[i] = (f[i-1] * i) % M
    end
    return f
end

############################################################
# T(p) = round(e * p!) mod M
############################################################

# integer encoding of e (safe for mod-arithmetic trick)
const E_INT = 2_718_281_828_459_045

function T_p(p::Int, fact)
    return (E_INT % MOD) * fact[p] % MOD
end

############################################################
# Compute C(N) using:
#
#     f(p) = (1/p) * Σ_{d|p} μ(d) * T(p/d)
#     C(N) = Σ f(p)
############################################################

function compute_C(N::Int)
    fact = factorials_mod(N, MOD)
    C = 0

    for p in 1:N
        s = 0

        # sum over divisors d of p
        for d in 1:p
            if p % d == 0
                s = (s + mobius(d) * T_p(div(p, d), fact)) % MOD
            end
        end

        f_p = (s * invmod(p, MOD)) % MOD
        C = (C + f_p) % MOD

        println("p=$p   C(p)=$C")
    end

    return C
end

############################################################
# MAIN
############################################################

println("Computing C($MAXN) mod $MOD ...")

result = compute_C(MAXN)

println("\nFINAL RESULT: C($MAXN) mod $MOD = $result")
