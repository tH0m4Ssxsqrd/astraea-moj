+++
title = "CodeForces 1346"
description = "See full list of contests held"
date = "2024-16-09"
aliases = ["contests"]
toc = false
+++
{{< submit-button >}}

# A.
# B.
# ...
# H.
# I. Pac-Man 2.0

- **time limit per test:** `3 seconds`
- **memory limit per test:** `256 megabytes`
- **input:** `standard input`
- **output:** `standard output`

Polycarp is developing a new version of an old video game "Pac-Man". Though he really enjoyed playing the original game, he didn't like some aspects of it, so he decided to alter the rules a bit.
In Polycarp's version, you play as Pac-Man and you have to collect pellets scattered over the game world while avoiding dangerous ghosts (no difference from the original yet). Polycarp didn't like the fact that there was no escape from the ghosts in the original, so, in his version, the game world is divided into n safe zones with m one-directional pathways between them — and it is guaranteed that Pac-Man can reach any safe zone from any other. Since safe zones are safe, the ghosts cannot attack Pac-Man while it is there, it is in danger only while traversing the pathways. Pac-Man starts the game in the safe zone s.

All pellets are scattered over the safe zones; initially, the i-th safe zone contains ai pellets (and if Pac-Man is in a safe zone, it may freely collect all the pellets in it). The pellets disappear after being collected, but after the last pellet in the game world is collected, new pellets spawn in the safe zones in the same quantity as before (ai new pellets spawn in the i-th zone). The pellets can be respawned any number of times, so the game is essentially infinite.

Polycarp has already determined the structure of the game world and the number of pellets in each safe zone. Now he is trying to find out if the game is difficult enough. There are q goals in the game, the i-th goal is to collect at least Ci pellets from the beginning of the game. Polycarp denotes the difficulty of the i-th goal as the minimum number of times the player has to traverse a one-directional pathway in order to collect Ci pellets (since only traversing a pathway puts Pac-Man in danger). If some pathway is traversed multiple times while Pac-Man is collecting the pellets, it is included in the answer the same number of times.

Help Polycarp to calculate the difficulty of each goal!

------

## Input:
The first line contains four integers n, m, q and s (2≤n≤15; n≤m≤n(n−1); 1≤q≤5000; 1≤s≤n) — the number of safe zones, the number of pathways, the number of goals and the index of the starting safe zone, respectively.

The second line contains n integers a1, a2, ..., an (1≤ai≤109), where ai is the initial number of pellets in the i-th safe zone (and the number of pellets that spawn in the i-th safe zone when the last pellet in the world is collected).

Then m lines follow, each line contains two integers vi and ui (1≤vi,ui≤n; vi≠ui) denoting a one-directional pathway from the safe zone vi to the safe zone ui. Each ordered pair (vi,ui) appears in this section at most once (there are no multiple pathways from vi to ui), and it is possible to reach every safe zone from every other safe zone using these pathways.

The last line contains q integers C1, C2, ..., Cq (1≤Ci≤1015), where Ci is the minimum number of pellets the player has to collect in order to fulfil the i-th goal.

------

## Output:
For each goal i, print one integer — its difficulty (the minimum number of times the player has to traverse along some pathway in order to collect at least Ci pellets).

------

# Exemplos:
## input
```
3 4 2 1
3 1 2
1 2
2 1
1 3
3 1
5 8
```
## output
```
1
3
```

------

## input
```
5 7 4 2
1 3 2 2 1
2 3
4 2
3 4
3 1
1 4
5 4
4 5
7 14 23 27
```
## output
```
2
6
10
13
```