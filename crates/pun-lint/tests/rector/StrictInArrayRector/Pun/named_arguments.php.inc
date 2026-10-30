<?php

// Rector appends a positional `true` after the named arguments, `in_array(needle: $i, haystack: [1], true)`, which
// PHP rejects ("Cannot use positional argument after named argument"). Pun names it.

function run(int $i)
{
    return in_array(needle: $i, haystack: [1]);
}
-----
<?php

// Rector appends a positional `true` after the named arguments, `in_array(needle: $i, haystack: [1], true)`, which
// PHP rejects ("Cannot use positional argument after named argument"). Pun names it.

function run(int $i)
{
    return in_array(needle: $i, haystack: [1], strict: true);
}
