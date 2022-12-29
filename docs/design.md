# Design

## Needs to support
- Calculations need to happen on all cores (number of threads == number of cpu's perfectly)
- The work done when calculating next move should build upon
work done previously
- At any point the best move should be possible to be returned
- Engine should by default calculate during oponent time


## Evaluator
Evaluation module should use neural network, but who would want to train it?
Use stockfish networks, library to evaluate a NNUE:
https://github.com/dshawul/nnue-probe
trained networks can be found here:
https://tests.stockfishchess.org/nns


## Opening books and endgame tables
Should be implemented, but this is not a priority
