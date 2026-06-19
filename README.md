# Visualisation generator for Bézier curve circle approximations

A small and quick project just for the fun of it. I was not trying to make it beautiful, I was just trying to have fun learning about all of this.

Thanks for the original idea to CaryKH's [video about imperfect circles in Adobe Animate](https://youtu.be/6eohAul-osM).

## Usage

1) ```cargo run --release``` will generate frames into output/ directory

2) ```./combine_png_into_video.sh``` will convert all those frames into a video using ffmpeg and save it as output.mp4 (or you could combine the frames in other ffmpeg wrapper of your choice)

## Configuration

Most of the things can be easily changed from the top of `main.rs` file. Specifically:
- Resolution
- Framerate
- Rendering duration
- Bezier `P1` and `P2` start and end distances
- Béziers sampling quality (how many in-between points are calculated when rendering)
- Number of Bezier curves used to approximate a circle
- Font size
- Circle radius

Changing them most wouldn't break anything, but I've tested only reasonable values, so beware

Also, if you're brave enough you can modify rendering order, text, colors, etc in the same `main.rs` file, but they aren't as easily changeable, and you'd have to understand my code
