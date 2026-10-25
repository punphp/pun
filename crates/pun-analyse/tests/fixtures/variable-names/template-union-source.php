<?php

namespace VariableNameTemplateUnionSource;

/**
 * @template Name of 'duplicate'
 * @param Name|'duplicate' $name
 */
function sameNameFromTwoTypes($name): void
{
    compact($name);
    echo $$name;
    $copy = $name;
    compact($copy);
    echo $$copy;
    $copy = 'existing';
    $existing = 1;
    compact($copy);
    echo $$copy;
}
