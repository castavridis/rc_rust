The temple of forever State Machine

R = red beads
G = green beads

(R, G)

1. E: if R ≥ 3: (r, g) -> (r - 3, g + 2)
2. S: (r, g) -> (g, r)

Theorem 1. No one ever leaves the Temple of Forever

(15, 12) S -> (12, 15)
(12, 15) E -> (9, 17)
(9, 17) S -> (17, 9)
(9, 17) E -> (12, 15)

Invariant
1. Total number of beeds never goes up
2. Changes to total: 0, -1
3. After first echange, nothing is divisable by 5 anymore?
4. Difference can only grow by 5 or 0 every time
5,5 = 0

27 beads total

(15, 12)

Invariant
If we have a state (x, y): 
f(x, y): (x - y) % 5

1. Exchange: f(E(x , y))

f(x-3, y+2)
= x - 3 -y - 2 % 5
= (x - y - 5) % 5
= (x - y) % 5

2. Swap: f(S(x, y))

= f(x-x+y, y-y+x)

note: modulus on a negative number is adding 5 until it becomes positive

