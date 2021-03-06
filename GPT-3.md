<a id="english">

# MFEK GPT-3 Policy
[日本語版を読む](#japanese)

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
<a id="japanese">

# エム・エフ・イー・Ｋ基金（ＭＦＥＫ）のＧＰＴ３型に関するポリシー
[Read in English](#english)

注：ＧＰＴ３型とは＜言語モデル＞と呼ばれる人工知能の一種である。

**オペン・ソース・プロジェクト cucoqu（三⇒曲⇐円⇒線⇐二　及び　ク・コ・キュ）の一部は２０２２年６月の現状のＧＰＴ３型によって書かれています。**

[ＧＰＴ３型](https://en.wikipedia.org/wiki/GPT-3)は株式会社 [OpenAI](https://openai.com/) のサービスである。

GitHub Copilot は使用されていません。 モジュール式字体エディタＫ基金ＮＰＯ法人（略語されているＭＦＥＫ基金の米国のModular Font Editor K Foundation Inc）は全ＡＰＩ呼び出しを資金提供しました。 **ク・コ・キュの場合は百ドル約の演算力が使用されました。**

エム・エフ・イー・Ｋ基金は以下を誓います：

* 生成された全てのコードは人間によってレビューされました。（今のところでその人間はフレッド・ブレンナンさんである。）
* 生成された全てのコメントは事実関係の正確性を確認し正確でない場合は削除されました。
* 生成された全てのコードは Rust（ラスト）コンパイラでテスト／リントされました。
* ＧＰＴ３型に「望み通りに音声言語からコードを書きなさい」形の指示はぜんぜん言わなかったです。寧ろ　ＧＰＴ３型に他の人が書いたソースコードが与えられて「それをラストへ 翻訳 しよう」と言う指示されました。
* 多くの関数は十回以上、数回は三十回以上ＧＰＴ３型へ送られます。

# ク・コ・キュ（`cucoqu`）固有ＧＰＴ３型使用情報

* 独自で翻訳する時間の４０〜４５パーセントをＧＰＴ３型使が節約してくれたと推測していますよ！
* コード翻訳はＣ＋＋（Skiaプロジェクトのソースコード）からラストにしました。 いく数間の類似性を見ることさえができれますよ。例えば [`<Conic as EvalTangentAt>::eval_tangent_at`](https://github.com/MFEK/cucoqu.rlib/blob/5f5451f1373a0cded2c559f8f0a6831c97c48810/src/co2qu/eval.rs#L28) と [`SkConic::evalTangentAt`](https://github.com/google/skia/blob/48e98da982d7535ce35dad0ecd7dfba591c4b9c4/src/core/SkGeometry.cpp#L1312) の２つの関数間を比較してみてください。
* ＧＰＴ３型には `SkGeometry.cpp`、`SkGeometry.h` のファイル内のいくつかの関数のみを与えられし、プロジェクトの終業間際に、`SkPath.cpp` から複数行を与えられました。
* 無修正結果ファイルの殆どは、ＧＰＴ３型の作成したＡＰＩらが醜すぎることを示す。 しかし、殆ど触れていないファイルを見たい場合は、[`src/coeffs.rs`](https://github.com/MFEK/cucoqu.rlib/blob/5f5451f1373a0cded2c559f8f0a6831c97c48810/src/coeffs.rs) を見てください。前述ファイルは（超！）稀に見る完璧ほぼ結果でしたよね。
