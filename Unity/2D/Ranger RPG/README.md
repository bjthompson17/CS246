# Overview
I created this simple game in Unity using the free [Tiny RPG - Forest](https://assetstore.unity.com/packages/2d/characters/tiny-rpg-forest-114685) Asset pack from the Unity Asset Store and some royalty free sound samples found on SoundBible.com. The purpose was for me to learn how to use Unity as a 2D game engine to build more complicated and interesting games. I think this was a success so far. Of course this is my first game made with the engine and I plan to continue to learn and improve on the skills I aquired creating this project.

Note: The Unity project folders are really big and contain many thousands of library, and temporary files that are irrelevent to the project. So I've only uploaded the "Assets" and "Project Settings" folders, which contain all of the relevant project info.

To open the project in Unity, download the entire "Ranger RPG" folder, then "Add" the folder as a project in the Unity Hub. You should then be able to open and play around with it. Unity will create all the nessecary missing files. Nothing will show at first because you need to add the "SampleScene" object in the folder "Assets/Scenes" to the game by dragging it from the project window into the Hierarchy. You can replace the default "Untitled Scene" by dragging over it.

# Environment Setup
It is recommeded that you simply install and use Visual Studio to use with Unity as it comes with the whole .NET environment already out of the box and requires no extra installs or setup. If you want to use VS Code like I did, however, there are instructions here: https://code.visualstudio.com/docs/other/unity 

My setup is as follows:

---
## Setting up .NET
You'll have to download and install the .NET Core and the .NET framework to get Visual Studio Code ready for C# development with Unity. Here's where you can download both:
https://dotnet.microsoft.com/download

## Visual Studio Code
---
There aren't many extensions you'll need for this setup. Just be sure to have VS Code installed. Here's some easy instructions for install:
- Simply go to https://code.visualstudio.com/ to download and run the VS Code installer for your system
- Once you've installed and opened VS Code on your computer, on the left side of the screen you should see some icons. Click the one that says "Extensions" when you hover over it. It should look like a box or boxes.
- The only extentsion you really need is the C# extension by Microsoft, but the Unity Debugger extension also seems handy.

---
## Unity Game Engine
- In order to start using Unity, you'll need to download and open the Unity Hub first from https://unity3d.com/get-unity/download (be sure to actually download the "Unity Hub"). You will find it very useful to create a free account, so if you don't have one already, make one. You should then be able to install the latest version of Unity from the Hub. Once your install is complete, you'll need to create a project also from the Unity Hub. At that point, you can open your project in Unity ... but the setup for VS Code isn't done yet. 
- From the "Window" tool menu, locate and open the "Package Manager" window. This will show you what extension packages are avaliable for Unity. Search for the "Visual Studio Code Editor" Package and make sure it is installed, if it's not, then install it. 
- Once that's done, go to the "Edit" tool menu and select "Preferences." Inside the preferences window, find the "External Tools" section on the left. You'll need to change from Visual Studio to Visual Studio Code via the "External Script Editor" dropdown menu. Since C# needs ".csproj" and ".sln" files generated to work with Visual Studio Code's intellisense, you'll need to select checkboxes to generate these files for at least the "Built-in Packages". "Local Packages" could also be needed if you install any scripting packages. At the bottom of the list, click the "Regenerate Project Files" button to make sure they are generated corrrectly. 
- Lastly, you need to launch VS Code from Unity, by opening a script file in the Unity Project window, in order to get IntelliSense to work. It may take a minute for Omnisharp to start so give it some time to load. If you have problems with IntelliSense not registering at any point, just close VS Code, go to the Preferences/External tools and regnerate project files, and then reopen your script file from Unity.

Unity is now ready to use with Visual Studio Code!

---
# The Game In Action
![](.img/In_Action.gif)

---
# Parts of Unity Used
For this project, I decided to focus on the 2D engine in Unity, mainly because I have plans for future 2D games. In my adventures with this platform, I ended up using a number of different tools provided with the 2D engine. Below is a list of systems that I used:
- Tilemaps: Used to create maps with ease using a tiled grid. Can be Square, Isometric, or Hexagonal.
- Physics2D Engine: Used to move the player, projectiles, and enemies and detect collisions with items and walls which have colliders attached. Just basicly anything physics related.
- UI System: Used to create "Pause" menu and show Item inventory and healthbars. 
- UnityEditor API: Used to implement custom functionality of tools in the Unity Editor itself through scripts. Really neat. Specifically, I made a tool to paint Unity Prefabs (Premade game objects) onto a tilemap. 
- Gizmos: A must have, easy to use debug feature, esspecialy when I need to visibly see range values of scripted objects.
- Audio: It was super easy to add sounds to the game with Unity's AudioSource component.
- Animator: Animation is absolutly essential to a remotly satisfying game. Unity's animator is pretty intuitive, similar to the animator in Blender, but with an added state machine controller to control multiple animations with game logic.
- Components: Unity's modular component system makes things really easy to work with and understand. Everything in a Scene is a GameObject. A GameObject has a transform to define position, rotation, and scale and can have any number of Components attached. For example, a Sprite Renderer component, which renders a defined sprite to the screen at the GameObject's transform. Scripts can be added as Components as well in the form of Monobehaviour classes.

# Useful Sites

Brackeys on YouTube is an absolute Unity Tutorial Legend. This is the best place to start learning Unity: https://www.youtube.com/channel/UCYbK_tjZ2OrIZFBvU6CCMiA

Unity provides their own useful tutorials here as well:
https://learn.unity.com/

The Unity Manual and Scripting API can be found here: https://docs.unity3d.com

Most of my technical questions that weren't answered by the API reference were answered well by other's Q&A's here: https://answers.unity.com/index.html
