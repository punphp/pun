<?php

// Missing second value; will trigger a runtime error.
$result = sprintf('Hello %s, you have %d messages', $name);
-----
4:18: Call to `sprintf` contains 2 placeholders, 2 values given.
