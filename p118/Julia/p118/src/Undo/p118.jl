module p118

#greet() = println("Hello World!")

function sieve(n)
    is_prime = [true for _ in 1:(n + 1)]
    primes = []
    greet2() = println(is_prime)
    greet3() = println(primes)
 #   is_prime[0] = false
 #   if n >= 1
 #       is_prime[1] = false
 #   end

    return primes
end

# приклад використання
n = 10
#prime = sieve(n)
greet () = println("Прості числа до ",n,":",sieve(n))




end # module p118

# Call the function after defining the module:
p118.greet()
p118.greet2()
p118.greet3()

