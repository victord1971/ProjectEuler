
using Primes, Combinatorics

# Усі цифри від 1 до 9
digits = collect(1:9)

# Допоміжна функція: перетворити вектор цифр у число
digits_to_int(v) = parse(Int, join(v))

# Генеруємо всі можливі розбиття множини {1,...,9} на групи цифр
function partitions(v)
    if length(v) == 1
        return [[v]]
    end
    result = []
    for i in 1:(length(v)-1)
        left = v[1:i]
        right = v[(i+1):end]
        for rest in partitions(right)
            push!(result, [left; rest])
        end
    end
    push!(result, [v])
    return result
end

# Перевірка чи всі частини в розбитті — прості
function is_valid_partition(p)
    nums = [digits_to_int(part) for part in p]
    all(isprime, nums)
end

# Основна функція
function euler118()
    seen = Set{Set{Int}}()
    perms = permutations(digits)
    for perm in perms
        for part in partitions(perm)
            # Пропускаємо, якщо числа мають 0 попередніх нулів (у нас їх немає)
            nums = [digits_to_int(a) for a in part]
            if all(isprime, nums)
                push!(seen, Set(nums))
                println(Set(nums))
            end
        end
    end
    return length(seen)
end

println("Кількість панцифрових простих множин: ", euler118())
