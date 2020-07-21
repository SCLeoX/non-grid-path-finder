# Rin's Non-grid Path Finder
This is a path finding algorithm for non-grid-based environments. It is guaranteed to find the shortest path between two given points. Essentially, it works by first constructing a navigation graph of obstacles, then using A* in that graph to find the shortest path.

## Limitations
- The traveling cost field must be uniform.
- All obstacles must be polygons.

## Efficiency
- Constructing the navigation graph from obstacles is `O(EV^2)`.
- Finding a path with a given constructed navigation graph is `O(EV)`.

\* `E` is the total number of edges in all obstacles; `V` is the total number of vertices in all obstacles.
