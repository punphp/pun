<?php

namespace CoverageMax;

function untyped($a)
{
    return $a;
}

function named(\stdclass $o): \stdClass
{
    return $o;
}
