<?php

function read(array $domains, array $locales): void
{
    $translations = getTranslations($domains, $locales);

    foreach ($translations as $namespace => $locales) {
        // $locales now refers to the loop value, original argument is lost
    }
}
-----
3:37: Parameter `$locales` is shadowed by a loop, catch, `global`, or `static` variable.
