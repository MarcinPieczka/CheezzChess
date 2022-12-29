# Design
## Goal of this chess engine
The goal is to create an engine that not only looks at objective
correctness of the play, but also judges the style.

To achieve this it will need to be pretty strong on it's own to be 
later able to sacrifice some of the chess performance, for chess style.

"Good" style in case of this engine would be to put additional value on material sacrifices,
so if the same position can be achieved with, or without a piece sacrafice, the two situations
will be evaluated differently. Other possibility is that with a sacrifice we achieve a worse position,
but depending on a configuration, we will sacrifise let's say 50 centipawns to just play more stylish.

### Needs to support
- Calculations need to happen on all cores (number of threads == number of cpu's perfectly)
- (Maybe later) The work done when calculating next move should build upon
work done on previous moves.
- At any point the best move should be possible to be returned
- (Maybe later) Engine should by default calculate during oponent time


### Evaluator
Evaluation module should use neural network, but who would want to train it?
Use stockfish networks, library to evaluate a NNUE:
https://github.com/dshawul/nnue-probe
trained networks can be found here:
https://tests.stockfishchess.org/nns


### Opening books and endgame tables
Should be implemented, but as UCI supports UI part to apply the books,
this is not a priority


## Algorithm

1. Receive UCI go message.
2. Create e.g 2 (the number may vary) levels of (distinct) moves which would become roots of a forest.
   These roots will establish a queue of work to be done, and would specify the depth to which it
   should be processed.
3. Spawn as many threads as there are cpus, and each thread should take one subroot
   at a time from the queue and process it.
4. A/B value for main root should be available and mutable for all threads
5. When time comes for choosing a move, the best moves should already be available


## Milestones
1. Single threaded A/B pruned min-max engine that calculates moves up to predefined depth.
2. Add multithreading.
3. Use Stockfish NNUE.
4. Judge the style (Maybe this should be done earlier, as this is main goal of the project).
5. Deepening the search after initial search to predefined depth.
