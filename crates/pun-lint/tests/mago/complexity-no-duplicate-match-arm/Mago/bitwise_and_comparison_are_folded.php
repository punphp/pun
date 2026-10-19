<?php

$result = match ($value) {
    1 => 1 << 2,    // 4
    2 => 2 ** 2,    // 4    - duplicate of arm 1
    3 => 2 + 2,     // 4    - duplicate of arm 1
    4 => 2 > 1,     // true
    5 => true,      // true - duplicate of arm 4
};
