<?php

function read(array $domains, array $locales): void
{
    $translations = getTranslations($domains, $locales);

    foreach ($translations as $namespace => $namespaceLocales) {
        // $locales is still accessible
    }
}
-----
