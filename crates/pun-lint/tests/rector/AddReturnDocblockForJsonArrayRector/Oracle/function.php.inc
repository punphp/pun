<?php

function decode_it(string $contents): array
{
    return json_decode($contents, true);
}
-----
<?php

/**
 * @return array<string, mixed>
 */
function decode_it(string $contents): array
{
    return json_decode($contents, true);
}
