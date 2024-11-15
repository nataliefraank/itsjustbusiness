# groupproject

Will Domm, John Saluta, Natalie Frank
October 18, 2024
Systems Programming

**Project Proposal**

**Overview**
	Our project will be a two-dimensional video game. We will integrate crates that allow for animated characters, background, and movement, such as ascending_graphics, blue_engine, sdl2, caper, and/or frug. These crates allow for integration of design features without sacrificing the integrity of our code nor the time it may take to code this independently. Game development in Rust is still in early phases, which doesn’t leave us with a foundational framework to go off of, but there are many elements we can use to start our project. For instance, games like Veloren use Rust to successfully build off of other successful video games and create a large player platform. Our game will be a 2D platforming-based game, with features such as scoring, precise movement-based gameplay, and a high-score mechanic. We also will have multiple textures to indicate multiple playable characters. 

**Required Learning**
	We will need to learn how to use different crates and APIs to create a professional and complete video game. This may include sign-ins, progress saving, and user interface additions. Reading, understanding, and becoming familiar with these crates will be an essential part of making our game. This will help us practice reading and writing off of code that we did not ourselves make, which will be a useful skill for our later computer science courses. We will need to make a 2D game engine that satisfies our needs, and build off that by incorporating ideas such as objects and game instances. 
	There are multiple helpful resources including game resource libraries and video game company frameworks that can define the trajectory and progress of our project. This includes Unity documentation and the ggez Rust library. These resources can help guide 2D drawing, sounds and animations, resource loading, and event handling. We will seek to facilitate seamless integration of crates and APIs to make our video game as professional and fun as possible. 

**Proposed Schedule**
_Week
Task_


10/28/24
We will write a Game Design Document (GDD) that outlines the game’s goals, mechanics, and features. This will also include basic character and environmental designs. 
We will create a shared file to document weekly changes and updates to the game.
We will research successful 2D game engines. 
We will set up the github repository. We will implement an API to test game rendering. 


11/04/24
We will use an API to receive mouse and keyboard input. 
We will implement player movement. We will use a physics crate.
We will create a player sprite. We will create 2D game assets.
We will set up a game loop that uploads and renders the player.


11/11/24
We will add collision detection between the player and objects. 
We will build a basic design level using tilemaps or custom geometry. 
We will add game mechanics like item collection or enemies. We will build items to collect or gain (i.e. coins, weapons, food). We will build an AI enemy that may chase or fight the player.


11/18/24
We will add sound and visual effects. We can use crates or APIs to integrate this. Sound and visual effects may be cued at various events, movements, or game mechanics. We will chart this out to ensure reliability and continuity within the game. 
We will add a basic user interface for health or victory points. Health and victory points can be seen throughout the game.
We will create multiple levels and transitions. These levels will encourage the user to continue playing the game and level up as the game progresses. Enemies, item collection, and/or points may instigate this.
We will create a game over and title screen.
We will have the status report submitted. The game should run enough to let the player(s) accomplish the main objectives. 


11/25/24
We will test the game and fix bugs. We will playtest the game to detect errors.
We will polish game mechanics, interactions, and features.
We will refine animations and sound effects.


12/02/24
We will optimize game performance. 
We will write game documentation. 
We will create a game demo for our final presentation. 


12/09/24
We will put finishing touches on the project, as well as debugging and preparing for our presentation. Our project will be complete and free of any errors that may prevent playability. We will optimize the user experience and accessibility to allow all players to participate in the project. 


12/16/24
We will practice our final presentations. We will present on our project.

**Status Update**
John and Natalie drafted a GDD (https://docs.google.com/document/d/1-JhXKA7o_2BysKqky-wLEJrhXI38WQaop78RezyCiPc/edit?usp=sharing). It includes fundamental game design ideas and mechanics. It does not go into much depth. We will add to the document as we develop.
We will track weekly changes and updates on GitHub commits right now because it has built-in version control. As the game becomes more advanced, this may change.
We found a repository of successful 2D games. 
We might use GFX to help with 2D game rendering. Reddit has more ideas.
We found the 2d physics crate Rapier and added it to the project repository.
We added the game development crate bevy and created a simple implementation.
John created the player sprite. Natalie created the tilemap.  
