<?php

// Rector writes `new \Collection(...)` for `collection(...)`, which PHP rejects ("Cannot create Closure for new
// expression"). Pun leaves a first-class callable alone.

$f = collection(...);
