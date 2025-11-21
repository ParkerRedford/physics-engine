# Intro and why
This project is mainly experimental. I read that physics engines use Euler's or Runge-Kutta 4 methods to simulate physics. In perspective, both methods are computationally heavy where Euler can require 15,000 FLOPs whereas RK4 can require 60,000 FLOPs for each particle, dot, pixel, or object; on top of that, both methods are non-deterministic. I believe I can reduce that count down to around 300-1000 FLOPs max by having the physics engine actually solve these math equations symbolically (like a compiler) to precompute everything from kinematics, graphics, and special effects at runtime. If the engine can't find a closed-form of an integral, it'll use a complex analysis or a numerical analysis pathway to keep the operations count low. I don't know what to expect from this project except achieve higher performance rates in magnitudes. Here are the GPUs I will be testing on to compare and contrast these physics engines: 9800 XTX, 2080 RTX, R9 M380, Geforce 670M, and Intel UHD Graphics 620.

# Stage 1
The first stage of the project is to make a function that takes a math equation (both real and complex) and output its derivative, partial derivative, or integral. The part where the compiler does its magic. I will build a small website app to help debug the equations for later use for the physics engine.

There are some missing components from our current symbolic mathematical solutions where there is a solution from academia. One such missing component is regarding with the Taylor and Macluarin series. When I use Google, AI, or Mathematic the best they give me is the Macluarin seires of a math equation instead of the Taylor series. Here are a few examples of the ones they give me.

<img width="413" height="71" alt="Screenshot 2025-10-13 184710" src="https://github.com/user-attachments/assets/17537ad2-b765-4ee7-89bb-4813b261d6f0" />
<img width="367" height="67" alt="Screenshot 2025-10-13 193320" src="https://github.com/user-attachments/assets/138aee2f-ec9f-4788-800e-e797c7a6f19a" />

However, this is what I am asking for.

<img width="706" height="67" alt="Screenshot 2025-10-13 184700" src="https://github.com/user-attachments/assets/cfb9d1a7-7f43-4212-93b0-0db9955bd334" />
<img width="733" height="121" alt="Screenshot 2025-10-13 193335" src="https://github.com/user-attachments/assets/c9775c90-31b5-4667-b468-c231da7bc806" />

Yes, you can get the correct Taylor series of these two equations from Google or AI, but the example equations are well-known and are already solved by a human. Custom equations are generally inaccurate from Google and AI; they tend to switch to the Macluarin series (and sometimes still inaccurate) because I believe the Taylor series too complicated for these engines. Mathematica gave me an overload error on the Taylor series, so that is not very helpful. I tend for this project to be the solution to that problem, and have it run locally and directly put into the physics engine itself. Furthermore, complex analysis functions aren't wildly available, so I want to implement them too. They can help solve the improper integrals too, so that is a plus.

Another missing component is that CAS (computer algebra system) programs don't typically allow you to input custom variables like that 'a' from the first example. They usually only allow the input variable function like 'x' and constant variables such as e, pi, or a custom constant variable. The CAS program doesn't output custom letter variables from a derivative or integral; instead, it outputs the number that contains the value of the variables.

- [ ] Real analysis
  - [ ] Derivative
  - [ ] Partial derivative
  - [ ] Integral
  - [ ] Taylor series
- [ ] Complex analysis
  - [ ] Gaussian
  - [ ] Error function
  - [ ] Signum function
  - [ ] Cauchy-Riemann
- [ ] Numerical analysis
  - [ ] Chebyshev

# Stage 2
The second stage of the project is to build the core physics engine, so that developers can interface with the engine.

- [ ] Classical mechanics
- [ ] Energy
- [ ] Thermodynamics
- [ ] Lights and optics

# Stage 3
The third stage is to build a variety of things related to simulations and FX.

- [ ] Particle system
- [ ] Fluid dynamics
- [ ] Soft-body system
- [ ] Rigid-body system
- [ ] Springs
- [ ] Blobs
- [ ] Render engine
- [ ] Scene-as-math for Signed Distance Functions (rendering technique)

# Stage 4
Not sure yet. Probably build an Unreal/Unity plugin to interface with this engine.

Timeframes: No idea.
