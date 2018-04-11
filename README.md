# Rust Console Barcode Generator

My first rust application.

Generates an CODE128 barcode in the console for the specified command-line argument -- just what you've always wanted!

Note that the barcode is inverted with additional 'quiet' areas either side for use in a light-on-dark terminal.  Change the `inverted` switch to reverse for dark-on-light.

To start:

```bash
cargo run "Hello World!"
```

**Note:** *GitHub*'s rendered Markdown line-spacing is likely to be wider than your terminal (so it shouldn't have gaps between the lines in your terminal).

Output (not inverted, assuming dark-on-light):

```
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌     
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌     
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌     
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌     
```

Output (inverted, assuming light-on-dark):

````
█████ ▌█▐█▌▐█▐▐█▐ █▐█▌▐▌▌██ █▐▐█▌█▌ ▐▐ ▌▐▌▐▌ ▌█▌▐▐█  ▌▌█▐▌ ▐ █▐▐█▌██▐▌▐ █ ▌▐▌ ▌█▌▐ █▌ ▌▌▐████▌
█████ ▌█▐█▌▐█▐▐█▐ █▐█▌▐▌▌██ █▐▐█▌█▌ ▐▐ ▌▐▌▐▌ ▌█▌▐▐█  ▌▌█▐▌ ▐ █▐▐█▌██▐▌▐ █ ▌▐▌ ▌█▌▐ █▌ ▌▌▐████▌
█████ ▌█▐█▌▐█▐▐█▐ █▐█▌▐▌▌██ █▐▐█▌█▌ ▐▐ ▌▐▌▐▌ ▌█▌▐▐█  ▌▌█▐▌ ▐ █▐▐█▌██▐▌▐ █ ▌▐▌ ▌█▌▐ █▌ ▌▌▐████▌
█████ ▌█▐█▌▐█▐▐█▐ █▐█▌▐▌▌██ █▐▐█▌█▌ ▐▐ ▌▐▌▐▌ ▌█▌▐▐█  ▌▌█▐▌ ▐ █▐▐█▌██▐▌▐ █ ▌▐▌ ▌█▌▐ █▌ ▌▌▐████▌
````
