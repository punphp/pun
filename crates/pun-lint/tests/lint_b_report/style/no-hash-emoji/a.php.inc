<?php

#️⃣ a comment
# plain
#️⃣[Attr]
function f() {} #️⃣ trailing
// #️⃣ not hash
-----
3:1: Emoji-based hash (`#️⃣`) used instead of ASCII `#`.
5:1: Emoji-based hash (`#️⃣`) used instead of ASCII `#`.
6:17: Emoji-based hash (`#️⃣`) used instead of ASCII `#`.
