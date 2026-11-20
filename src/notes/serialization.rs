//! # BLS12-381 serialization
//!
//! The following rules describe G1 and G2 point encodings.
//!
//! * $\mathbb{F}\_p$ elements are encoded in big-endian form. They occupy 48
//!   bytes in this form.
//! * $\mathbb{F}\_{p^2}$ elements are encoded in big-endian form, meaning that
//!   the $\mathbb{F}\_{p^2}$ element $c\_0 + c\_1 \cdot u$ is represented by the
//!   $\mathbb{F}\_p$ element $c\_1$ followed by the $\mathbb{F}\_p$ element $c\_0$.
//!   This means $\mathbb{F}_{p^2}$ elements occupy 96 bytes in this form.
//! * The group $\mathbb{G}\_1$ uses $\mathbb{F}\_p$ elements for coordinates. The
//!   group $\mathbb{G}\_2$ uses $\mathbb{F}_{p^2}$ elements for coordinates.
//! * $\mathbb{G}\_1$ and $\mathbb{G}\_2$ elements can be encoded in uncompressed
//!   form (the x-coordinate followed by the y-coordinate) or in compressed form
//!   (just the x-coordinate). $\mathbb{G}\_1$ elements occupy 96 bytes in
//!   uncompressed form, and 48 bytes in compressed form. $\mathbb{G}\_2$
//!   elements occupy 192 bytes in uncompressed form, and 96 bytes in compressed
//!   form.
//!
//! The most-significant three bits of a $\mathbb{G}\_1$ or $\mathbb{G}\_2$
//!   encoding should be masked away before the coordinate(s) are interpreted.
//!   These bits are used to unambiguously represent the underlying element:
//! * The most significant bit, when set, indicates that the point is in
//!   compressed form. Otherwise, the point is in uncompressed form.
//! * The second-most significant bit indicates that the point is at infinity.
//!   If this bit is set, the remaining bits of the group element's encoding
//!   should be set to zero.
//! * The third-most significant bit is set if (and only if) this point is in
//!   compressed form _and_ it is not the point at infinity _and_ its
//!   y-coordinate is the lexicographically largest of the two associated with
//!   the encoded x-coordinate.
//!
//! ## Raw field encodings
//!
//! The SP1 patch also exposes raw field byte methods with a different format.
//! `Fp2::to_bytes` and `Fp2::from_bytes` use c0 followed by c1, each encoded
//! as 48 big-endian bytes. G2 point encodings use c1 followed by c0 instead.
//!
//! `Fp6` byte methods use c0, c1, then c2, each in the raw Fp2 format.
//! `Fp12` byte methods use c0 then c1, each in the raw Fp6 format.
//! Each raw encoder and its matching decoder use the same coefficient order.
