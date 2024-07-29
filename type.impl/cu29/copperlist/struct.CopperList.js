(function() {var type_impls = {
"cu_caterpillar":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BorrowDecode%3C'__de%3E-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-BorrowDecode%3C'__de%3E-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'__de, P&gt; <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html\" title=\"trait bincode::de::BorrowDecode\">BorrowDecode</a>&lt;'__de&gt; for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload + <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html\" title=\"trait bincode::de::BorrowDecode\">BorrowDecode</a>&lt;'__de&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.borrow_decode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.borrow_decode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html#tymethod.borrow_decode\" class=\"fn\">borrow_decode</a>&lt;__D&gt;(decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __D</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;, <a class=\"enum\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/error/enum.DecodeError.html\" title=\"enum bincode::error::DecodeError\">DecodeError</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecoder.html\" title=\"trait bincode::de::BorrowDecoder\">BorrowDecoder</a>&lt;'__de&gt;,</div></h4></section></summary><div class='docblock'>Attempt to decode this type with the given <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html\" title=\"trait bincode::de::BorrowDecode\">BorrowDecode</a>.</div></details></div></details>","BorrowDecode<'__de>","cu_caterpillar::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#54\">source</a><a href=\"#impl-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#56\">source</a><h4 class=\"code-header\">pub fn <a href=\"cu29/copperlist/struct.CopperList.html#tymethod.new\" class=\"fn\">new</a>(id: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>, payload: P) -&gt; <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;</h4></section><section id=\"method.change_state\" class=\"method\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#64\">source</a><h4 class=\"code-header\">pub fn <a href=\"cu29/copperlist/struct.CopperList.html#tymethod.change_state\" class=\"fn\">change_state</a>(&amp;mut self, new_state: <a class=\"enum\" href=\"cu29/copperlist/enum.CopperListState.html\" title=\"enum cu29::copperlist::CopperListState\">CopperListState</a>)</h4></section><section id=\"method.get_state\" class=\"method\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#68\">source</a><h4 class=\"code-header\">pub fn <a href=\"cu29/copperlist/struct.CopperList.html#tymethod.get_state\" class=\"fn\">get_state</a>(&amp;self) -&gt; <a class=\"enum\" href=\"cu29/copperlist/enum.CopperListState.html\" title=\"enum cu29::copperlist::CopperListState\">CopperListState</a></h4></section></div></details>",0,"cu_caterpillar::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-Debug-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + CopperListPayload,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","cu_caterpillar::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-Decode-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html\" title=\"trait bincode::de::Decode\">Decode</a> for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload + <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html\" title=\"trait bincode::de::Decode\">Decode</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.decode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html#tymethod.decode\" class=\"fn\">decode</a>&lt;__D&gt;(decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __D</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;, <a class=\"enum\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/error/enum.DecodeError.html\" title=\"enum bincode::error::DecodeError\">DecodeError</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decoder.html\" title=\"trait bincode::de::Decoder\">Decoder</a>,</div></h4></section></summary><div class='docblock'>Attempt to decode this type with the given <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html\" title=\"trait bincode::de::Decode\">Decode</a>.</div></details></div></details>","Decode","cu_caterpillar::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-Encode-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encode.html\" title=\"trait bincode::enc::Encode\">Encode</a> for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload + <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encode.html\" title=\"trait bincode::enc::Encode\">Encode</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.encode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encode.html#tymethod.encode\" class=\"fn\">encode</a>&lt;__E&gt;(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __E</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/error/enum.EncodeError.html\" title=\"enum bincode::error::EncodeError\">EncodeError</a>&gt;<div class=\"where\">where\n    __E: <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encoder.html\" title=\"trait bincode::enc::Encoder\">Encoder</a>,</div></h4></section></summary><div class='docblock'>Encode a given type.</div></details></div></details>","Encode","cu_caterpillar::CuList"]],
"cu_wt901_tester":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BorrowDecode%3C'__de%3E-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-BorrowDecode%3C'__de%3E-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'__de, P&gt; <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html\" title=\"trait bincode::de::BorrowDecode\">BorrowDecode</a>&lt;'__de&gt; for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload + <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html\" title=\"trait bincode::de::BorrowDecode\">BorrowDecode</a>&lt;'__de&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.borrow_decode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.borrow_decode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html#tymethod.borrow_decode\" class=\"fn\">borrow_decode</a>&lt;__D&gt;(decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __D</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;, <a class=\"enum\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/error/enum.DecodeError.html\" title=\"enum bincode::error::DecodeError\">DecodeError</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecoder.html\" title=\"trait bincode::de::BorrowDecoder\">BorrowDecoder</a>&lt;'__de&gt;,</div></h4></section></summary><div class='docblock'>Attempt to decode this type with the given <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.BorrowDecode.html\" title=\"trait bincode::de::BorrowDecode\">BorrowDecode</a>.</div></details></div></details>","BorrowDecode<'__de>","cu_wt901_tester::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#54\">source</a><a href=\"#impl-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload,</div></h3></section></summary><div class=\"impl-items\"><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#56\">source</a><h4 class=\"code-header\">pub fn <a href=\"cu29/copperlist/struct.CopperList.html#tymethod.new\" class=\"fn\">new</a>(id: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>, payload: P) -&gt; <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;</h4></section><section id=\"method.change_state\" class=\"method\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#64\">source</a><h4 class=\"code-header\">pub fn <a href=\"cu29/copperlist/struct.CopperList.html#tymethod.change_state\" class=\"fn\">change_state</a>(&amp;mut self, new_state: <a class=\"enum\" href=\"cu29/copperlist/enum.CopperListState.html\" title=\"enum cu29::copperlist::CopperListState\">CopperListState</a>)</h4></section><section id=\"method.get_state\" class=\"method\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#68\">source</a><h4 class=\"code-header\">pub fn <a href=\"cu29/copperlist/struct.CopperList.html#tymethod.get_state\" class=\"fn\">get_state</a>(&amp;self) -&gt; <a class=\"enum\" href=\"cu29/copperlist/enum.CopperListState.html\" title=\"enum cu29::copperlist::CopperListState\">CopperListState</a></h4></section></div></details>",0,"cu_wt901_tester::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-Debug-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + CopperListPayload,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","cu_wt901_tester::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Decode-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-Decode-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html\" title=\"trait bincode::de::Decode\">Decode</a> for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload + <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html\" title=\"trait bincode::de::Decode\">Decode</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.decode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html#tymethod.decode\" class=\"fn\">decode</a>&lt;__D&gt;(decoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __D</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;, <a class=\"enum\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/error/enum.DecodeError.html\" title=\"enum bincode::error::DecodeError\">DecodeError</a>&gt;<div class=\"where\">where\n    __D: <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decoder.html\" title=\"trait bincode::de::Decoder\">Decoder</a>,</div></h4></section></summary><div class='docblock'>Attempt to decode this type with the given <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/de/trait.Decode.html\" title=\"trait bincode::de::Decode\">Decode</a>.</div></details></div></details>","Decode","cu_wt901_tester::CuList"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Encode-for-CopperList%3CP%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#impl-Encode-for-CopperList%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encode.html\" title=\"trait bincode::enc::Encode\">Encode</a> for <a class=\"struct\" href=\"cu29/copperlist/struct.CopperList.html\" title=\"struct cu29::copperlist::CopperList\">CopperList</a>&lt;P&gt;<div class=\"where\">where\n    P: CopperListPayload + <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encode.html\" title=\"trait bincode::enc::Encode\">Encode</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.encode\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/cu29/copperlist.rs.html#47\">source</a><a href=\"#method.encode\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encode.html#tymethod.encode\" class=\"fn\">encode</a>&lt;__E&gt;(&amp;self, encoder: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut __E</a>) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"enum\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/error/enum.EncodeError.html\" title=\"enum bincode::error::EncodeError\">EncodeError</a>&gt;<div class=\"where\">where\n    __E: <a class=\"trait\" href=\"https://docs.rs/bincode/2.0.0-rc.3/bincode/enc/trait.Encoder.html\" title=\"trait bincode::enc::Encoder\">Encoder</a>,</div></h4></section></summary><div class='docblock'>Encode a given type.</div></details></div></details>","Encode","cu_wt901_tester::CuList"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()