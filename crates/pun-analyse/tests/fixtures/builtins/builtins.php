<?php

function builtins(): void
{
    \Closure::getCurrent();
    array_first([1, 2]);
    grapheme_levenshtein('a', 'b');
    pg_service(null);
    curl_multi_get_handles(curl_multi_init());
    get_error_handler();
    new \IntlListFormatter();
    new \Filter\FilterFailedException();
    \Locale::isRightToLeft('en');
    new \CurlSharePersistentHandle();
    array_last([1, 2]);
}
