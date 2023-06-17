# 💥 Kamikaze Joe 
Kamikaze Joe is a fully on-chain PvP arena game. Players compete against each other as elite kamikaze agents, willing to blow themselves up to kill their governments' enemy. Carefully manage your energy, rapidly evaluate your opponents moves and decide when to active your bomb to kill everyone around you. 

This repository contains the necessary code and assets to run the game, which includes both the state and logic on-chain, as well as a Unity client used for engine rendering.

# :gear: Game Mechanics 
In Kamikaze Joe, players find themselves in an arena where they must strategically navigate and outmaneuver their opponents. The game revolves around managing energy to make movements and gain an advantage. Each movement in the game costs energy, and players can only move if they have enough energy left.

Players must decide how much energy they want to spend on a move, selecting a value from 1 to 5. The chosen energy expenditure determines the number of squares the player will move, provided there are no obstacles in the way. Strategic decision-making is key, as players must balance the need for speed and positioning with the limited energy available

The game map features chests scattered throughout, which serve as energy rechargers. By reaching and interacting with these chests, players can replenish their energy reserves, enabling them to make additional moves. 

When they feel they are close enough to their opponents, agents can blow themselves up and try to kill their enemy in the ultimate sacrifice. It's crucial to plan movements and activate the bomb carefully to outlast and outwit your opponents.

# :wrench: Technical Details 

Kamikaze Joe demonstrates that even with the same throughput of the blockchain, it is possible to achieve visually different speeds. The game achieves this by utilizing the Unity engine for rendering, which allows for smooth and visually engaging gameplay experiences.

The repository includes the following components:

- Program: contains the game implementation for Solana 

- Unity Client: Contains the Unity project that serves as the game engine and handles the rendering of the game world. The Unity client interacts with the program deployed on the blockchain, allowing players to see the game state in real-time and make moves accordingly.

# 💣 How to play

- 1 to 5 to select energy level
- WASD to move
- SPACE to activate bomb

# :page_with_curl: License 
This project is licensed under the MIT License. Feel free to modify and distribute the game according to the terms of the license.
