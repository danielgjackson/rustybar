# Rust Console Barcode Generator

My first rust application.

Generates an CODE128 barcode in the console for the specified command-line argument -- just what you've always wanted!

Note that the barcode is inverted with additional 'quiet' areas either side for use in a light-on-dark terminal.  Change the `inverted` switch to reverse (and remove the quiet areas) for dark-on-light.

Uses "CODE B" only, so works with ASCII characters 32-128, and is not as efficient with numbers as using "CODE A" would be.

To start:

```bash
cargo run "Hello World!"
```

Output (not inverted, assuming dark-on-light):

```
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌
     █▐ ▌ ▐▌ ▌▌ ▌█ ▌ ▐▌▐▐  █ ▌▌ ▐ ▐█▌▌█▐▌▐▌▐█▐ ▐▌▌ ██▐▐ ▌▐█▌█ ▌▌ ▐  ▌▐▌█ █▐▌▐█▐ ▐▌█ ▐█▐▐▌
```
