<?php

$result = match ($value) {
    1 => !23,   // false
    2 => false, // false - duplicate of arm 1
};
