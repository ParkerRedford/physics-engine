Project: A deterministic physics engine that uses an AST/DAG algorithm to create symbolic math to put everthing on the GPU for the SIMD instruction set. The CPU will still be required for some physics stuff (probably collisions) because GPUs can't do conditions well.

The first stage is to make a function that takes a math equation and output its derivative, partial derivative, or integral. I will build a small website app to help debug the equations for later use for the physics engine.

The second stage is to build the core physics engine, so that developers can interface with it.

The third stage is probably build a render engine.

The fourth stage is whatever right now. Probably build a plugin.

Timeframes: No idea. Maybe something in December.
