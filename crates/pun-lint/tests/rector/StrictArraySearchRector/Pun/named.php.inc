<?php

// Pun: Rector adds `true` after the named arguments, which PHP rejects, so the call stays.
array_search(needle: $a, haystack: $b);
