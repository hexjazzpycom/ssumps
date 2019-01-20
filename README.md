ssumps
======

It is a rust implementation of pseudo-polynomial time algorithm for [the subset sum problem](https://en.wikipedia.org/wiki/Subset_sum_problem).

The implementation lazily creates bit table and accumulates intermediate results in additional vector. It restores result subset from the intermediate results vector, when necessary bit is found.

Improved implementation handles bit table in batches. It stores batch in u64 type and process batches via bitwise operations. It allows to increase performance significantly.

You may find naive implementation in function `find_subset_naive` and improved implementation in function `find_subset`.


Short description of subset sum algorithm.
==========================================

The first step is the initialization of bit table.

    X - input set 
    n - input set length
    s - input sum
    T - bit table
    
    for i in 1..n
        for j in 1..s 
            T(0, 0) = 1
            T(i, 0) = 1 if i >= 1
            T(0, j) = 0 if j >= 1

For `X=[3, 7, 1, 0, 9, 4, 3, 5, 8]` and `s=17` init bit table state looks like:

![Initial bit table view](https://i.imgur.com/7H34jtK.png)

Bit table values calculation:

    for i in 1..n
        for j in 1..s 
            T(i, j) = T(i-1, j), when j < X[i]
            T(i, j) = 1, when j = X[i]
            T(i, j) = max(T(i-1, j), T(i, j - X[i])), when j > X[i]

For `X=[3, 7, 1, 0, 9, 4, 3, 5, 8]`, `s=17`, `i=2`, `j=10`: `X[i] = 7` and `T(2, 10) = 1`

![Cell calculation scheme](https://i.imgur.com/JnFh01Y.png)


Calculated bit table view:

![Calculated bit table view](https://i.imgur.com/NiXrgxt.png)

Subset restoring, the result is `[9, 1, 7]`: 

![Subset restoring](https://i.imgur.com/tCa4l80.png)

Naive algorithm visualisation. 
==============================

For `input_set = [3, 7, 1, 0, 9, 4, 3, 5, 8]` and `input_sum = 17`.

![Naive algorithm visualisation](https://i.imgur.com/QWkht2v.gif)


