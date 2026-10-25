<?php

namespace ArrayAssignmentCertainty;

function rootIndex(): void
{
    if (isset($root)) {}
    $root['nested'][$root] = [];
    $root;
}
