# MFEK GPT-3 Policy

**Parts of this project were written by a type of artificial intelligence, a language model called GPT-3, as that model existed in the month of June 2022.**

[GPT-3](https://en.wikipedia.org/wiki/GPT-3) is a project of [OpenAI LP](https://openai.com/).

GitHub Copilot was not used. The MFEK Foundation Inc. funded API calls. **For `cucoqu`, ≈$100 in compute was used.**

MFEK pledges:

* All code generated was reviewed by a human (in practice Fred Brennan).
* All comments generated were reviewed for factual accuracy and removed if inaccurate.
* All code generated was tested and linted by a real compiler.
* GPT-3 was not told to just write code. It was given human-produced code and told to _translate_ it to Rust.
* Many functions are sent back to GPT-3 more than ten times; a few more than thirty.

# `cucoqu` specific GPT-3 information

* I estimate that GPT-3 saved me 40%–45% of the time I'd spend translating this on my own.
* The translation was from Skia C++ to Rust. You can really see the similarity between functions; e.g. compare [`<Conic as EvalTangentAt>::eval_tangent_at`](https://github.com/MFEK/cucoqu.rlib/blob/5f5451f1373a0cded2c559f8f0a6831c97c48810/src/co2qu/eval.rs#L28) to [`SkConic::evalTangentAt`](https://github.com/google/skia/blob/48e98da982d7535ce35dad0ecd7dfba591c4b9c4/src/core/SkGeometry.cpp#L1312).
* I only gave GPT-3 functions in the files `SkGeometry.cpp`, `SkGeometry.h`, and a bit of `SkPath.cpp`.
* Most files heavily show my influence as the APIs GPT-3 made were ugly. However, if you'd like to see a file I barely touched, take a look at [`src/coeffs.rs`](https://github.com/MFEK/cucoqu.rlib/blob/5f5451f1373a0cded2c559f8f0a6831c97c48810/src/coeffs.rs).
