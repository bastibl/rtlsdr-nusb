# Source provenance

This crate is distributed under Apache-2.0. Upstream attribution is consolidated
in `NOTICE`; source files deliberately omit repeated copyright headers.

## Hardware implementation

Reference: [jtarrio/webrtlsdr](https://github.com/jtarrio/webrtlsdr/tree/5699cec220cb0349e8f9144b7b71d3d03b5d9dbf),
revision `5699cec220cb0349e8f9144b7b71d3d03b5d9dbf`, Apache-2.0.
The relevant upstream source headers identify Jacobo Tarrio Barreiro (2024)
and Google Inc. (2013). The reference itself credits Osmocom's hardware
register research and Google Chrome Radio Receiver as its origin.

| Reference source | Rust adaptation |
| --- | --- |
| `src/rtlsdr/rtlcom.ts` | `src/protocol.rs`: USB register spaces, byte order, demodulator synchronization, I2C and GPIO |
| `src/rtlsdr/rtl2832u.ts` | `src/rtl2832u.rs`: initialization, FIR coefficients, rate/correction/IF dividers, direct sampling |
| `src/rtlsdr/r8xx.ts` | `src/tuners/mod.rs`, `src/tuners/registers.rs`: initialization, calibration, gain, tracking filters and PLL |
| `src/rtlsdr/r820t.ts`, `src/rtlsdr/r828d.ts` | `src/tuners/mod.rs`: probing, input selection and Blog V4 upconversion |

These are modified Rust adaptations, not verbatim copies. Changes include:

- Native nusb transport with owned, lazy operations and structured errors.
- Ordered tuner probing, including awaiting the R828D identity read.
- A PLL unlock error after both attempts fail, rather than reporting success.
- Integer divider calculations, validated configuration boundaries, and tuner
  shadow registers updated only after successful writes.
- Consistent Blog V4 low-band selection and upconversion at the 28.8 MHz edge.
- GPIO output latch programmed before output enable, and independent best-effort
  cleanup of receiver, bias power, tuner and demodulator on shutdown.
- Complete configuration invalidation and reinitialization after failed or
  canceled control sequences.
- Persistent USB receive buffers, caller-owned float output and borrowed raw IQ.

No GPL implementation files from librtlsdr, rtl-sdr-rs or rtlsdr-next's benchmark
references were incorporated. This is a port from the Apache-licensed reference,
not a claim of clean-room implementation.

## Device and stream architecture

Reference: [bastibl/hydrasdr-rs](https://github.com/bastibl/hydrasdr-rs/tree/9bbda5110cf1cda672445904d3ab4c3ad247bf97),
revision `9bbda5110cf1cda672445904d3ab4c3ad247bf97`, using its Apache-2.0 option.

`src/maybe_future.rs`, `src/errors.rs`, `src/usb/mod.rs`, `src/session.rs`,
`src/rx/` and `src/test_support.rs` reuse and adapt its operation combinators,
backend abstractions, shared ownership, exclusive stream claim, terminal/retryable
shutdown, consuming close, native/wasm drop behavior and persistent queue design.
The RTL-SDR version replaces HydraSDR firmware commands, packing and sample
conversion with RTL2832U register operations and unsigned eight-bit IQ conversion.
Multi-register control transactions use an owned hardware lease; overlapping
transactions return Busy rather than interleave I2C operations.
