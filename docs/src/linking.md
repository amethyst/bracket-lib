# Linking to Bracket-lib

The quickest and easiest way to use `bracket-lib` in your program is to include the entire library in your project. Open your project's `Cargo.toml` and include:

```toml
[dependencies]
bracket-lib = "0.8"
```

You now have the whole `bracket-lib` project linked to your program.

## Using the Github Version

If you'd like to live on the bleeding edge, you can link to the Github version. In your project's `Cargo.toml` file, add the following:

```toml
[dependencies]
bracket-lib = { git = "https://github.com/amethyst/bracket-lib.git" }
```

The main reason to do this is to try out new features. The Github version isn't *always* as stable as I'd like, so proceed with caution.