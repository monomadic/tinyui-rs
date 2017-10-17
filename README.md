<img src="tinyui-logo.png" alt="go-staticgen" height="75">

NOTE: This code is not ready for use yet. It can create basic things but the API changes frequently.

A tiny native windowing and GUI library for rust. Currently supports MacOS with other platforms in the future.

It does NOT use a render loop like winit and almost all other gui frameworks with rust do, but works closer with the native operating systems render loop and uses callbacks, so it is far better on cpu and lends itself more to long-running applications you don't want sucking your cpu in the background. It also feels a billion (quantified, honest) times nicer to work with than something like winit, which is turning into a slushy garbage dump as it progresses :D

Supports common controls like buttons and labels, and webkit (soon opengl) controls, natively, with absolute minimal dependencies. The library usage is loosely based on ggez.
