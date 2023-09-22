# The ant on the board (variant of the Langton's Ant)

This is test job.

## The task

There is a board of 1024 x 1024 white cells.
The ant located on the board in position (512, 512) and looks to the up.

The ant follows this rules:

- on a white cell it inverts the color of the cell, then turns clockwise, then moves forward the the next cell;
- on a black cell it inverts the color of the cell, then turns counterclockwise, then moves forward to the next cell.

The game ends when the Ant reaches end of the Board.

What program should do:

- resolve the ant's path;
- draw the path as BMP or PNG image with color depth 1, assuming that one cell is one pixel;
- calculate number of black cells on it;
- use at less RAM as possible.
