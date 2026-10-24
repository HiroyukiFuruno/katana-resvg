# KatanA resvg maintenance scope

This fork starts from upstream resvg v0.48.1. The original Apache-2.0 OR MIT
licenses and contributor attribution remain in effect.

The published packages are `katana-usvg` and `katana-resvg`; their Rust library
names remain `usvg` and `resvg`. This is a narrow distribution of three fixes:

- Preserve primary-font glyphs when a fallback font supplies missing glyphs.
- Include stroke bounds in path layer bounds, including axis-aligned lines.
- Resolve CSS `ex` units using the selected font's metrics.

Tracking: https://github.com/HiroyukiFuruno/katana-resvg/issues/1

## Maintenance and exit policy

At each KRR dependency update, compare the latest upstream release and main
branch with these patches. Reassess every patch before updating the base; a
clean application alone is not evidence of correctness. Preserve upstream
tests and add focused regressions for the changes above.

When an upstream release contains equivalent fixes, validate it against KRR's
rendering, font identity, performance, and memory requirements, then return to
the upstream packages. This fork does not promise an independent feature
roadmap or a faster upstream release schedule.

Related upstream discussion: https://github.com/linebender/resvg/issues/486
concerns shaping and fallback. It does not establish that all three changes
have already been reported or accepted upstream.
