# enigma-sim and the power of hashtables
Back in 2015 me and my friend @franOffi decided to build an Enigma simulator with decryption capabilities.
I was super happy with the results because we did it all ourselves and after a month or two of pair programming the project worked flawlessy.

Then i began university and I was taught hash tables.

That seemed an optimal project to test this data structure. 
During the brute-force decryption we score ciphertext based on the probabilities of quadgrams. In the first version, we organized strings and probabilities in arrays and the search was performed through for loops. It is quite clear that the complexity of that search function is O(M) where M is the number of quadgrams.
Unfortunately, we used a file that contained 400000 quadgrams.

If we use hash tables instead, the complexity of the search function is O(1) is the average case.
That alone makes a huge differences.

Indeed i simply ported the code to Rust and changed the scoring function and the execution time changed as follows:
(C old version based on measures by @franOffi on mid 2011 i5 iMac)
(Rust new version based on measures by me on i5 7200u)

| Number of letters | C time          | Rust time         |
|-------------------|----------------:|------------------:|
|10                 |1301s            |0.468s             |
|15                 |2260s            |0.682s             |
|20                 |3196s            |0.685s             | 
|30                 |5055s            |1.286S             |
|40                 |6986s            |1.686s             |
|127                |400 mins         |5.068s             |
