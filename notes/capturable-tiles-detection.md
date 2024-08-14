# Capturable tiles detection

**Capturable** tiles are those tiles, that can be captured upon placing a piece
on the board.

Capturable tiles are detected in groups and returned as sets of unique
positions.

A tile is considered capturable if it is empty or occupied by enemy building or
the Cathedral.

A set of tiles is capturable if it contains only empty tiles or only one
capturable building (which gets captured). If two or more capturable buildings
are found in a set, the set does not get captured.

It is easy to find separate tile groups with hashsets and some DFS algorithm
(4-connected Flood Fill does the trick). The problem is that not any found set
of tiles can be captured. Here's an example:
<pre>
 _____________
| b
|  [][][][]
|  []  a []
|  [][][][]
|
</pre>
Flood Fill would detect areas `a` and `b` but only `a` is capturable because of
the game rules.

The issue can be resolved with a check:
if there are two unique tiles contacting walls, then all areas are catpurable.
Like that:
<pre>
 _____________      _____________
| b  []            |b []
|  [][]        or  |[][]
|[][]   a     even |     a
|                  |
</pre>
Here, `a` and `b` can be catupured. And cases such this:
<pre>
 _____________
| b  []
|  [][][][]
|  []  a []
|  [][][][]
|
</pre>
are automatically resolved: only a single wall is contacted so `b` won't be
captured.

However, here is an ambiguous situation:
<pre>
 _____________
| b  [][]
|  [][][][]
|  []  a []
|  [][][][]
|
</pre>
Two tiles contact walls, yet `b` should not be capturable.
The fix: merge adjacent wall contacting tiles into a single contact somehow.

  - ~~In set of such tiles count pairs of tiles, that have Manhattan distance
    of 1.~~ Doesn't work with continuous groups of occupied tiles (> 2)

  - Use good old flood fill to find groups of tiles contacting walls