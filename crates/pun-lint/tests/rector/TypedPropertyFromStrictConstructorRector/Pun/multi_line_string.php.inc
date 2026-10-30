<?php

namespace App;

// Rector writes `@var (literal-string & lowercase-string & non-falsy-string)`, the accessory types of the value;
// Pun does not print those and leaves the property.
class MultiLine
{
    public $multi;

    public function __construct()
    {
        $this->multi = "a
b";
    }
}
