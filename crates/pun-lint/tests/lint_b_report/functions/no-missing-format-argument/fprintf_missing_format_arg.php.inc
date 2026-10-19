<?php

fprintf($handle, 'Hello %s %s', $name);
-----
3:8: Call to `fprintf` contains 2 placeholders, 2 values given.
