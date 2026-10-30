<?php

class Open
{
}

function run(object $o, Open $open)
{
    return !is_a($o ?: $open, Open::class);
}
-----
<?php

class Open
{
}

function run(object $o, Open $open)
{
    return !($o ?: $open) instanceof \Open;
}
