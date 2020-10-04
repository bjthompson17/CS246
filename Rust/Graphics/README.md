# Overview
This program is a fun little widget you can have going to entertain you while you work. It presents you with a smiling face that follows your mouse around the screen via gravitational physics. What kinds of fun tricks can you do with it?

The point of this program was to get me introduced to some more advanced graphics and window handling in Rust.

---
# Environment Setup
## Rust
---
This is programmed in the Rust Programming Language. You can learn more about it at [rust-lang.org](https://www.rust-lang.org/)
## Compiler Setup
---
I set my project up with Cargo. Cargo is a terminal Rust project manager that comes with rust. It automates the entire process of setting up a workspace, setting up a git repository in said workspace, linting your rust code, building complex programs, and even downloading library dependancies. You can download the rust compiler with cargo by following the install instructions on the [Rust language website](https://www.rust-lang.org/tools/install), and get started in Chapter 1 of the [Rust Programming Language Book](https://doc.rust-lang.org/book/ch01-00-getting-started.html).

Setting up a new project with Cargo is easy. Simply navigate in your terminal to the folder you wish to create your project and type the following command:
>cargo new [insert your project name]

This will make a new project directory with a "Cargo.toml" file, a "src" folder, and a git repository with .gitignore already inside to ignore the build files. All that's left to do is open the "Cargo.toml" file and make sure the info there is correct, otherwise make it correct. That's it!

## Dependencies 
---
For this project, I had to install some external libraries for window creation and graphics with OpenGL. It's pretty easy to do so with Cargo. All you need to do is search for the libraries you want on [crates.io](https://crates.io/), copy the dependency code from the site, and paste it into your "Cargo.toml" file under "\[dependencies]." Cargo automatically downloads the required files when it builds. 

The libraries I ended up using, and their uses, are listed below:
- **autopilot**: Cross platform automation library. I used this library to get the mouse location anywhere on the screen, even outside the window.
- **glium**: Cross platform graphics library that manages window creation through the winit library and OpenGL graphics through the glutin library. I had to use this library because it's the only cross platform graphics package I found that allowed me to make a window that is both transparent and always on top as well as setup a graphics environment with it.
- **image**: Provides basic image loading, saving, and manipulation functionality.
## Visual Studio Code
---
I used VS Code as an IDE for this program. It integrates fully with Cargo via the extension Rust by The Rust Programming Language. It's really easy to install and set up. 
- Simply go to https://code.visualstudio.com/ to download and run the VS Code installer for your system
- Once you've installed and opened VS Code on your computer, on the left side of the screen you should see some icons. Click the one that says "Extensions" when you hover over it. It should look like a box or boxes.
- Search extensions for Rust by The Rust Programming Language and install that one by clicking the green "Install" button.
- Once that's done, you'll need to reload your window by clicking on the blue "Reload" button on the extension.
- VS Code is now ready for Rust!
---
# The Program in Action
![](.img\action.gif)

---
# Known Issues
As of now, winit does not have a cross platform solution for click through windows, so while they aren't majorly in the way at the current size, the tiny windows will block mouse events from passing through them. You'll have to make due with throwing them out of the way when you want to click something.

---
# Useful Sites
- [Tutorials Point: Rust](https://www.tutorialspoint.com/rust/)
- [Crates io](https://crates.io/)
- [Using OpenGL](https://learnopengl.com/) (Specifically, learn how shaders work to understand this code)
## Library Docs
- [glium library Docs](https://docs.rs/glium/0.27.0/glium/)
- [glutin library Docs](https://docs.rs/glutin/0.25.0/glutin/) (used by glium to manage OpenGL context)
- [winit library Docs](https://docs.rs/winit/0.23.0/winit/) (used by glutin to create windows)
- [autopilot library Docs](https://docs.rs/autopilot/0.4.0/autopilot/)
- [image library Docs](https://docs.rs/image/0.23.10/image/)
