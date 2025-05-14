def calculate_a(m, n, a_table):
    """
    Calculates the value of A(m, n) using recursive rules and base cases.
    Uses memoization with a_table.
    """
    # Ensure table has at least m+1 rows
    while len(a_table) <= m:
        a_table.append([])

    # Ensure row m has at least n+1 columns
    while len(a_table[m]) <= n:
        a_table[m].append(None)

    # Check if the value is already calculated
    if a_table[m][n] is not None:
        return a_table[m][n]

    # Base cases
    if m == 0 and n == 0:
        result = 0
    elif m == 0 and n == 1:
        result = 1
    # Recursive rules
    elif m > 0 and n == 0:
        # Rule A(m,0) = A(m-1, 1) + A(m-1, 0) for m > 0
        # Derived from A(m,n) = A(m-1, n+1) + A(m-1, n) with n=0
        result = calculate_a(m - 1, 1, a_table) + calculate_a(m - 1, 0, a_table)
    elif m == 0 and n > 1:
        # Rule A(0,n) = A(1, n-1) - A(0, n-1) for n > 1
        # Derived from A(m-1, n+1) = A(m,n) - A(m-1, n) with m=1, n-1=n' => n=n'+1
        # A(0, n'+1) = A(1, n') - A(0, n')
        val1 = calculate_a(1, n - 1, a_table)
        val2 = calculate_a(0, n - 1, a_table)
        if val1 is not None and val2 is not None:
             result = val1 - val2
        else:
             result = None # Indicate failure in recursive call
    elif m > 0 and n > 0:
        # Rule A(m,n) = 2 * A(m, n-1) + A(m-1, n-1) for m > 0, n > 0
        val1 = calculate_a(m, n - 1, a_table)
        val2 = calculate_a(m - 1, n - 1, a_table)
        if val1 is not None and val2 is not None:
            result = 2 * val1 + val2
        else:
            result = None # Indicate failure in recursive call
    else:
        # Should not happen for m, n >= 0
        result = None # Indicate error or unhandled case

    # Store the calculated value if successful
    if result is not None:
        a_table[m][n] = result

    return result

# Initialize an empty table
a_table = []

# Call the function to calculate A(5, 5) and populate the table up to that point
calculate_a(5, 5, a_table)

# Print the generated table
print("Generated A(m,n) table up to m=5, n=5:")
for row in a_table:
    print(row)

