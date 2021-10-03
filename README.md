<p align="center"><img src="assets/logo.png" /></p>

*binocle* is a graphical tool to visualize binary data.
It colorizes bytes according to different rules and renders them as pixels in a rectangular grid.
This allows users to identify interesting parts in large files and to reveal image-like regions.

## Examples

| ELF binary | MS Teams memdump | Doom assets | `perf record` samples |
|---|---|---|---|
| <img src="assets/example-elf.png" width="200" /> | <img src="assets/example-teams-memdump.png" width="200" /> | <img src="assets/example-doom.png" width="200" /> | <img src="assets/example-perf-record.png" width="200" /> |

## How it works

The program allows you to control various parameters like the *offset* into the file, the *stride*,
the *width* of the rectangular grid as well as the way in which (groups of) bytes are colorized.

<p align="center"><img src="assets/binary-view.png" /></p>

## Screenshots

![](assets/screenshot.png)

## Related work

  - [A Visual Study of Primitive Binary Fragment Types](http://www.rumint.org/gregconti/publications/taxonomy-bh.pdf)
  - [binvis.io](http://binvis.io/)
  - [cantor.dust](https://sites.google.com/site/xxcantorxdustxx/) ([talk](https://www.youtube.com/watch?v=4bM3Gut1hIk))
