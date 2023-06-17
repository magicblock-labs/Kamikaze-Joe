# 💥 Kamikaze Joe 
Kamikaze Joe is a fully on-chain tactical PvP arena game. Players compete against each other as elite kamikaze agents, members of shadowy organizations willing to blow themselves up to kill their governments' enemy. Carefully manage your energy, evaluate your opponent's moves and decide when to commit the ultimate sacrifice to kill everyone around you.

This repository contains the necessary code and assets to run the game, which includes both the on-chain state and logic, and a Unity client used for rendering.

# :gear: Game Mechanics 
In Kamikaze Joe, players find themselves in an arena where they must strategically outmaneuver their opponents. Every Agent starts with 100 energy. Players can select a range from 1 to 5 and move across the board, consuming the corresponding amount of energy. The chosen energy expenditure determines the number of squares the player will move, provided there are no obstacles in the way. Strategic decision-making is key, as players must balance the need for speed and positioning with the limited energy available. The game map features Rechargers scattered throughout. By reaching Rechargers, players replenish their energy reserves, enabling them to make additional moves. 
When they feel they are close enough to their opponents, agents can activate their bomb to instantly kill the opponent. Bomb autoinflict 20 damage to energy. Bombs have a 1 square range around you.  

The last Agent standing wins the game. 

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
