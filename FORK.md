# KatanA resvg maintenance scope

The current release candidate is based on upstream resvg v0.49.0
(`66ee0dc7d787d7e0b0ab3085b7849f78838cf2ba`). Its candidate package versions
are `katana-usvg` and `katana-resvg` `0.49.0-katana.1`; the Rust library names
remain `usvg` and `resvg`. This candidate is still under validation and has not
been published or adopted by KRR. The earlier v0.48 merge and publication
attempt remain historical evidence and do not establish a successful release.
The original Apache-2.0 OR MIT licenses and contributor attribution remain in
effect.

The approved fork-specific scope remains limited to these three fixes:

- Preserve primary-font glyphs when fallback-font shaping supplies missing
  glyphs.
- Include stroke bounds in path layer bounds, including axis-aligned lines.
- Resolve CSS `ex` units using metrics from the physical font selected by the
  resolver.

Upstream v0.49.0 already includes dominant-baseline inheritance through nested
text elements. That upstream behavior is retained in this candidate and is
separate from the three approved fork-specific fixes.

Tracking: https://github.com/HiroyukiFuruno/katana-resvg/issues/1

## Maintenance and exit policy

At each KRR dependency update, compare the latest upstream release and main
branch with these patches. Reassess every patch before updating the base; a
clean application alone is not evidence of correctness. Preserve upstream
tests and add focused regressions for the changes above.

When an upstream release contains equivalent fixes, validate it against KRR's
rendering, font identity, performance, and memory requirements, then return to
the upstream packages. The 0.49.0 candidate has not yet passed the formal
quality and performance gates or completed publication; permission to publish
the fork does not itself establish product adoption. This fork does not
promise an independent feature roadmap or a faster upstream release schedule.

Related upstream discussion: https://github.com/linebender/resvg/issues/486
concerns shaping and fallback. It does not establish that all three changes
have already been reported or accepted upstream.
