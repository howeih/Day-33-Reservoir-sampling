Day 33: Reservoir sampling
<br>
Reservoir sampling(https://en.wikipedia.org/wiki/Reservoir_sampling) is super useful when there is an endless stream of data and your goal is to grab a small sample with uniform probability.
<br>
The math behind is straightforward. Given a sample of size K with N items processed so far, the chance for any item to be selected is K/N. When the next item comes in, current sample has a chance to survive K/N*N/(N+1)=K/(N+1) while the new item has chance K/(N+1) to be selected.