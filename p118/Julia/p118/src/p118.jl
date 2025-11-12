
function sieve_of_eratosthenes(n::Int)
    # Створення масиву булевих значень
    is_prime = trues(n)
    is_prime[1] = false  # 1 не є простим числом

    for i in 2:floor(Int, sqrt(n))
        if is_prime[i]
            for j in i^2:i:n
                is_prime[j] = false
            end
        end
    end

    # Повертаємо список простих чисел
    return findall(is_prime)
end

# Виклик функції
n = 1_000_000_000
primes = sieve_of_eratosthenes(n)
println("Прості числа до $n: ", primes[end])


