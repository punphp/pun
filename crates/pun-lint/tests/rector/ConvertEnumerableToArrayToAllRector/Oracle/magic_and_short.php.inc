<?php

namespace App;

use Illuminate\Support\Collection;

/**
 * @param Collection<int, string> $names
 * @param Collection<string> $short
 */
function run(Collection $names, Collection $short)
{
    $names->undefinedMethod();
    $names->toArray();
    $short->toArray();
}
-----
<?php

namespace App;

use Illuminate\Support\Collection;

/**
 * @param Collection<int, string> $names
 * @param Collection<string> $short
 */
function run(Collection $names, Collection $short)
{
    $names->undefinedMethod();
    $names->all();
    $short->toArray();
}
