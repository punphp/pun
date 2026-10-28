<?php

function process(array $data): void
{
    echo $data['a'];
}
-----
<?php

/**
 * @param array<string, mixed> $data
 */
function process(array $data): void
{
    echo $data['a'];
}
