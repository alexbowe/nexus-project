# nexus-project


## Introduction

This project is a proof-of-concept implementation of the RSA encryption 
algorithm in rust, targeting the RISC-V architecture for execution on 
the [Nexus Zero Knowledge Virtual Machine](https://nexus.xyz).

It creates a 64 bit RSA key by using [ChaCha8](https://loup-vaillant.fr/tutorials/chacha20-design) to generate random prime
numbers using the [Rabin-Miller probabilistic primality algorithm](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test).

*Note: This code is not fit for production cryptography.*

## Motivation

The Nexus VM allows computations to be proven and verified in a
distributed manner, enabling trustless outsourced computing. The ramifications of this are myriad, but more information can be found on their [website](https://nexus.xyz) and in their [whitepaper](https://nexus-xyz.github.io/assets/nexus_whitepaper.pdf).

For me, this project is intended to become acquainted at developing for
the zkVM, but also to become acquainted with Rust.

RSA was chosen since it is relatively simple to implment, albeit
non-trivial, it sticks to the cryptographic theme, and has been
compromised in the past (the NSA paid to add a vulnerability to the random number generator - [source](https://www.reuters.com/article/world/exclusive-nsa-infiltrated-rsa-security-more-deeply-than-thought-study-idUSBREA2U0TY/)).

Although an unknown vulnerability wouldn't be detected by the zkVM (it 
proves that code was executed as intended), I can envision users 
wanting proof that a service or executable is not using
known-to-be-compromised code (i.e. is actually running the code 
that they publish or have audited).

## Lessons

This exercise was illuminating in many ways. Here are the most
salient things that I experienced:

- Rust's trait system is a great way to support the
[Interface Segregation Principle](https://en.wikipedia.org/wiki/Interface_segregation_principle).
- The Nexus zkVM supports a subset of RISC-V, which meant that
the standard library had to be disabled. This led me to learn
how to import crates with stdlib-dependant features disabled, and posed
some difficulty when trying to seed the random number generator.
- This raised some interesting questions. For example, if the VM
supported the `seed` register (as detailed [here](https://link.springer.com/article/10.1007/s13389-021-00275-6)), is it provable that the
register has not been manipulated? Is it okay if they computer
running the code knows your random seed? Does non-determinism hinder provability?
- This also meant that panics did not display a stack trace, which made it difficult to debug (although tracing with print statements worked fine).

## Future Work

- Accept a seed from the client via the private input tape by using the internal API ([example](https://github.com/nexus-xyz/nexus-zkvm/blob/main/api/examples/prover_run.rs)). Or utilize distributed random
beacons (such as [Strobe](https://sonnino.com/papers/strobe.pdf), by Don Beaver) or randomness ledgers.
- Increase the rounds of ChaCha to 20, and increase the RsaKey to 4096
bits at minimum (the current settings are insecure).
- Handle all checked big-integer arithmetic more carefully (currently
all of it is assumed to work, which can lead to panics with different inputs).
- Use generics to make it possible to change the encryption key length
programmatically (Rust appears to have great support for this, but time
was a factor).
- Add unit tests and documentation.
- Find a way to override the panic handler to display a stack trace
(it seems that a panic handler can be registered once per process, but it might be possible to modify the process memory to point to ours instead). Or compile and run in a stdlib-enabled environment for testing.

## How to Run

Install Rust, the RISC-V target, and the Nexus zkVM by following
the instructions [here](https://github.com/nexus-xyz/nexus-zkvm).
Then run these commands:

```
cargo nexus run    # Compile and execute, generating a trace
cargo nexus prove  # Generate a proof of the execution trace
cargo nexus verify # Verify the proof
```
