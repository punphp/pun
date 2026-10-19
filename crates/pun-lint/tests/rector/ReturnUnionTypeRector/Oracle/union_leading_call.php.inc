<?php

function a(bool $b)
{
    strlen('a');
    if ($b) {
        return 1;
    }
    return 'x';
}
-----
<?php

function a(bool $b): int|string
{
    strlen('a');
    if ($b) {
        return 1;
    }
    return 'x';
}
