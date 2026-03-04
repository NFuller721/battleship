# Review Time!

Finished the project at 9:17 PM with one known issue. If you attempt to stack
ships nothing stops you. Overall, barring that one problem, I think I did quite
well.

# Checking my work

At this point I will begin to make more changes to the binary, however these
will be to ensure that the code works. With that in mind I am going to change
this crate to a library and add a testing binary.

## Logic Bugs

There were a few unexpected logic bugs. The first of which being setting the
ships to `None` while setting a new ship. After that I discovered that the
messages saying what the action only displayed "player one" instead of "player-
two". Finally there were two issues in the hit logic for the `Ship` structure.

## Design Issues

There were a few things that did not match my proto-spec defined in DESIGN.md.
First being that there is no `Board::new` it was replaced with `Board::builder`.
In my opinion this makes more since with the way I wrote the code. The next
is that you must define the first player in the game (e.g.
`Game::<PlayerOne>::new()`). This is fine for now, but is a simple fix that
would be changed in the future. Lastly and most frustratingly is the fact that
game does not mutate but returns it self. Due to this you need to constantly
redefine the game variable.

# Notes

If this were a longer project I would have wanted a better way to check against
hits or ship stacking. Right now we just loop through all of the ships, but I
imagine there are much better ways of handling this.

# Response

This is the response after running `cargo run --bin main`.

```txt
player one hit player two's ship at (2, 9)
player two hit player one's ship at (2, 9)
player one hit player two's ship at (3, 9)
player two missed at (3, 9)
player one hit player two's ship at (4, 9)
player two missed at (4, 9)
player one hit player two's ship at (5, 9)
player two missed at (5, 9)
player one sunk player two's ship at (6, 9)
player two missed at (6, 9)
```
