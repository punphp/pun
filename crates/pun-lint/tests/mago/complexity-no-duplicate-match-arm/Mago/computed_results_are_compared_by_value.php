<?php

$result = match ($value) {
    1 => 1 + 2,  // 3
    2 => -(-3),  // 3 - duplicate of arm 1
};
