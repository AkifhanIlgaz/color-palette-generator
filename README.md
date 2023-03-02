Basic command-line program that extracts the color palette of an image by using k-means algorithm

# How it works

First, you have to clone this repository on your local machine and run cargo with two arguments:

- path to your image
- number of colors in color palette

_Example_:

```bash
cargo run images/samurai.jpg 5
```

_output:_

![samurai](https://github.com/AkifhanIlgaz/color-palette-generator/blob/master/images/output/samurai_output.JPG)

# TODO

- [ ] Write automated tests

- [ ] Improve terminal output

- [ ] Convert it to fully functional CLI program

- [ ] Made optimizations on k-means algorithm

- [ ] Give additional flags to print other color representations

- [ ] Accept URLs

- [ ] Convert output to JSON for API uses
