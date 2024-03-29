# Appendix

This appendix contains additional information about some concepts that are not covered in the main documentation.

## Main vs. auxiliary execution trace segments (`main` and `aux`)

With Randomized AIRs, the construction of the execution trace can be split into multiple rounds, with the verifier providing new randomness between rounds. In the first round, before any random values are available, the Prover builds the first segment of the execution trace, which we refer to as the `main` trace (using Winterfell terminology). Additional trace segments can be built after the prover has committed to the first execution trace segment. When building auxiliary trace segments, the prover has access to the extra randomness sent by the verifier (in the non-interactive version of the protocol, this randomness is derived from the previous trace segment commitments). We refer to these as auxiliary trace segments (using Winterfell terminology again), and we call the first one the `aux` segment in AirScript and throughout this documentation.

Miden VM only makes use of one auxiliary trace segment, and in Winterfell the number of auxiliary trace segments is currently limited to one. Therefore, AirScript currently only supports the main execution trace called `main` and a single auxiliary trace segment called `aux`.