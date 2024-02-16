42 project done in colaboration with [bielaltes](https://github.com/bielaltes) 

Usage: cargo run -- [-f FILENAME] [-g SIZE] [-a ALGORITHM] [-m METHOD] [-he HEURISTIC] [-o MOVEMENTS]

    - [-f | --file] [filename]

    - [-g | --generate] [size]

    - [-a | --algorithm] [a_star | ida_star]
        a_star: Faster algorithm but requires more memory and may lead to a notable slowness of the computer.
        ida_star: Slower algorithm based on astar that does not requires that much memory but takes longer time to get the solution.

    - [-m | --method] [normal | greedy | uniform]
        normal: knows the previous steps and the cost of the path.
        greedy: only knows the cost of the path.
        uniform: only knows the previous steps.

    - [-he | --heuristic]
        manhattan: sum of the distances of the tiles to their goal positions.
        hamming: number of tiles in the wrong position.
        euclidean: sum of the squares of the distances of the tiles to their goal positions.
        linear_conflicts: sum of the manhattan distances and the number of conflicts.
        NoAdmisible: twice the manhattan distance.

    - [-o | --override] 
        This flag allows the execution of 5 or higher puzzle sizes.

    By defaut, the program will run with the following parameters: ida_star, normal, manhattan.
    Also, for protecting computer resources, the program will not allow higher than 4x4 puzzles without specifying the maximum number of movements to check (-o).
