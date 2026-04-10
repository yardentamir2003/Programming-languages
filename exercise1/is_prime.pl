% Define smallest prime numbers: 2,3
is_prime(2).
is_prime(3).

% For any number bigger than 3 check
is_prime(P) :-
    P > 3,

    % check number is not even, since 2 is the only even prime number
    P mod 2 =\= 0,

    % Check number does not have other as divisors starting from 3    
    \+ has_divisor(P, 3).

% has_divisor(N, D)

% Base Case - if N divided by D has a remainder of 0, D is a divisor
has_divisor(N, D) :-
    N mod D =:= 0.

% Recursion, If N is not divisible by D try the next odd number
has_divisor(N, D) :-
    D * D < N,
    New_D is D + 2,
    has_divisor(N, New_D).
    