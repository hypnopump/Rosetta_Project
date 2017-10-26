# First Hashing Experiment

### The Problem

Ask the user for an input and apply the sha-2 (sha-512) hash on it. Store it in a variable.

Ask the user for the same input. Hash it the same way and compare it to the previous variable. If the hash matches, return a "Good job!". If it does not, return a "Try again." and allow a retry.

### Input >> Desired Output / Test Cases

Hello -> 3615f80c9d293ed7402687f94b22d58e529b8cc7916f8fac7fddf7fbd5af4cf777d3d795a7a00a16bf7e7f3fb9561ee9baae480da9fe7a18769e71886b03f315 + Hello -> 3615f80c9d293ed7402687f94b22d58e529b8cc7916f8fac7fddf7fbd5af4cf777d3d795a7a00a16bf7e7f3fb9561ee9baae480da9fe7a18769e71886b03f315 = Good job!

Hashing Practice -> a40fac78c9002b340f42026375f3726472208e2e1bfbde4ecb38ba2daa8c6caa3c52cad985d3230b1d784c2a26508ebf0f6b80400334febca56bd5ff8b1dce86 + Hashing Practice -> a40fac78c9002b340f42026375f3726472208e2e1bfbde4ecb38ba2daa8c6caa3c52cad985d3230b1d784c2a26508ebf0f6b80400334febca56bd5ff8b1dce86 = Good job! 

Hello Privacy -> 1cc8ced45c45d97604d1c43587bdb2627038074ac90ce809a0e85da6d63bedc5a3be8c81188cc5f6262c15bf0100b866b984c67f113a7c5203db0cffea19c57c + Hello -> 3615f80c9d293ed7402687f94b22d58e529b8cc7916f8fac7fddf7fbd5af4cf777d3d795a7a00a16bf7e7f3fb9561ee9baae480da9fe7a18769e71886b03f315 = Try again.
