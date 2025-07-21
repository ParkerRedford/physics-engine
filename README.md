This graphics library will power a physics simulation that could simulate 1-30 million particles (plus interact with them) with either an AMD, Intel, Nvidia, (or embedded) graphics card. Key takeaway: We don't need a dedicated GPU to simulate PhysX; in fact, PhysX with a dedicated GPU can be a burden.

Note: I won't be open-sourcing the math nor physics portion of the project because the physics is deterministic (using the Taylor series), an ODE solver (partially), differentiate equations symbolically (using an AST), is GPU-accelerated, runs at a low level langauge, and a semi-compiler for physics. My assumption is that this project is exactly what the government, defense, military, education, engineers, institutions, science, academia, Linux community, Valve, and video game companies would want.

I will create some demos, apps, and videos to showcase my philosophy.
