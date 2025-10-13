Project: A deterministic physics engine that uses an AST/DAG algorithm to create symbolic math to put everything on the GPU for the SIMD instruction set. The CPU will still be required for some physics stuff (probably collisions) because GPUs can't do conditions well. However, expect about 1-3 million particles in a fluid simulation (with collisions) on any type of graphics card versus 100,000 - 300,000 using PhysX on a dedicated card.

The first stage is to make a function that takes a math equation (both real and complex) and output its derivative, partial derivative, or integral. I will build a small website app to help debug the equations for later use for the physics engine.

The second stage is to build the core physics engine, so that developers can interface with it.

The third stage is to build a render engine.

The fourth stage is probably to build a machine learning model on the engine, so we can interface with it using English. The model will not be from PyTorch.

The fourth stage is whatever right now. Probably build a plugin for Unreal and/or Unity.

Timeframes: No idea.
