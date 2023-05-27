# Building With Bevy Workbook: Crabber

##### snen.dev

This is the first commit of the Building With Bevy Workbook, which follows along
the [Building With Bevy blog series](https://blog.snen.dev/building-with-bevy).
Each commit in this repository represents a snapshot of the code at the end of
each post (and potentially other useful checkpoints as well).

This commit adds a online multiplayer networking to `crabber` using
[NAIA](https://github.com/naia-lib/naia). To do so, it breaks `crabber` into
three distinct `protocol`, `server`, and `app` crates.

Read
[Building With Bevy Part 0](https://blog.snen.dev/building-with-bevy/00-introduction)
to start from the beginning, and
[Building With Bevy Part 3](https://blog.snen.dev/building-with-bevy/02-implementing-crabber)
to read about this commit.
