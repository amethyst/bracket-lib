# What is Bracket-lib

`bracket-lib` started out life as RLTK - The RogueLike Toolkit. Initially, I needed a code-page 437 terminal library on which to build roguelikes. The [Rust Roguelike Tutorial](http://bfnightly.bracketproductions.com/rustbook/) grew, and so did the library. Then [Hands-on Rust](https://hands-on-rust.com/) (my book about learning Rust) was born, and `bracket-lib` became the primary library behind the book.

Bracket-lib is intended to fulfill the following needs:

* A user-friendly *teaching* library. If I have to choose between performance and a new-user friendly interface, 99% of the time I'll choose the friendly approach.
* Provide virtual consoles on a variety of platforms, from the web to major Operating Systems.
* Provide a number of extensions to allow you to do layering, sprites, and more advanced rendering---without hurting the overall goal of the library.
* Provide a good "on ramp" for moving onto high-performance libraries once you've mastered the basics.
* Remain agnostic about how you write your game. ECS, tick methods, embedded scripting---take the path you want. Bracket-lib is meant to provide the basic tools you need and let you unleash your imagination.

This "getting started" guide is intended to help you get started with `bracket-lib`.