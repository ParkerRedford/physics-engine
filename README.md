Project: A deterministic physics engine that uses an AST/DAG algorithm to create symbolic math to put everything on the GPU for the SIMD instruction set. The CPU will still be required for some physics stuff (probably collisions) because GPUs can't do conditions well. However, expect about 1-3 million particles in a fluid simulation (with collisions) on any type of graphics card versus 100,000 - 300,000 using PhysX on a dedicated card because the hardware isn't spending all its resources on approximations.

The first stage of the project is to make a function that takes a math equation (both real and complex) and output its derivative, partial derivative, or integral. I will build a small website app to help debug the equations for later use for the physics engine.

There are some problems even with our current symbolic mathematical solutions can't solve. For instance, when I try to ask for the Taylor series for e^x, programs usually give me the Maclurian series instead. Like this.

<img width="413" height="71" alt="Screenshot 2025-10-13 184710" src="https://github.com/user-attachments/assets/b44b22a4-369b-49e2-8ed8-e03f79500bf2" />

But this is what I am asking for, so I have to manually write the equations myself.

<img width="706" height="67" alt="Screenshot 2025-10-13 184700" src="https://github.com/user-attachments/assets/57a875db-db63-4ef8-adb6-74fd143ffb2c" />

My expected goal for this library is to solve that issue.


The second stage of the project is to build the core physics engine, so that developers can interface with the engine.

The third stage is to build a render engine.

The fourth stage is probably to build a machine learning model on the engine, so we can interface with it using English. The model will not be from PyTorch.

The fourth stage is whatever right now. Probably build a plugin for Unreal and/or Unity.

Timeframes: No idea.
