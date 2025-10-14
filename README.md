Project: A deterministic physics engine that uses an AST/DAG algorithm to create symbolic math to put everything on the GPU for the SIMD instruction set. The CPU will still be required for some physics stuff (probably collisions) because GPUs can't do conditions well. However, expect about 1-3 million particles in a fluid simulation (with collisions) on any type of graphics card versus 100,000 - 300,000 using PhysX on a dedicated card because the hardware isn't spending all its resources on approximations.

The first stage of the project is to make a function that takes a math equation (both real and complex) and output its derivative, partial derivative, integral, or Taylor series. I will build a small website app to help debug the equations for later use for the physics engine.

There are some missing components from our current symbolic mathematical solutions where there is a solution from academia. One such missing component is regarding with the Taylor and Macluarin series. When I use Google, AI, or Mathematic the best they give me is the Macluarin seires of a math equation instead of the Taylor series. Here are a few examples of the ones they give me.

<img width="413" height="71" alt="Screenshot 2025-10-13 184710" src="https://github.com/user-attachments/assets/17537ad2-b765-4ee7-89bb-4813b261d6f0" />
<img width="367" height="67" alt="Screenshot 2025-10-13 193320" src="https://github.com/user-attachments/assets/138aee2f-ec9f-4788-800e-e797c7a6f19a" />

However, this is what I am asking for.
<img width="706" height="67" alt="Screenshot 2025-10-13 184700" src="https://github.com/user-attachments/assets/cfb9d1a7-7f43-4212-93b0-0db9955bd334" />
<img width="733" height="121" alt="Screenshot 2025-10-13 193335" src="https://github.com/user-attachments/assets/c9775c90-31b5-4667-b468-c231da7bc806" />

Yes, you can get the correct Taylor series of these two equations from Google or AI, but the example equations are well-known and are already solved by a human. Custom equations are generally inaccurate from Google and AI; they tend to switch to the Macluarin series (and sometimes still inaccurate) because I believe the Taylor series too complicated for these engines. Mathematica gave me an overload error on the Taylor series, so that is not very helpful. I tend for this project to be the solution to that problem, and have it run locally and directly put into the physics engine itself. Furthermore, complex analysis functions aren't wildly available, so I want to implement them too. They can help solve the improper integrals too, so that is a plus.


The second stage of the project is to build the core physics engine, so that developers can interface with the engine.

The third stage is to build a render engine.

The fourth stage is probably to build a machine learning model on the engine, so we can interface with it using English. The model will not be from PyTorch.

The fourth stage is whatever right now. Probably build a plugin for Unreal and/or Unity.

Timeframes: No idea.
