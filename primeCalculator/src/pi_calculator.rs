/*
    max_value = 1000000

    primes = new list

    primes[] = 2
    print prime
    primes[] = 3
    print prime

    first candidate = 5

    while candidate < max_value
        if candidate isPrime
            primes[] = candidate
            print prime
        endif
        candidate += 2
        if candidate isPrime
            primes[] = candidate
            print prime
        endif
        candidate += 4
    endwhile



    function isPrime - candidate
        isPrime = true
        sqrt = square root of candidate

        for all primes
            val = prime
            if val > sqrt
                break
            else if $candidate % val == 0 AND candidate != val
                isPrime = false
                break
            endif
        endfor

        return isPrime
    endfunc
*/

fn main() {
    
}