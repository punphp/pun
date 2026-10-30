<?php

namespace ReturnContract;

function stop(): never
{
    throw new \RuntimeException('stop');
}

function number(): int
{
    return 42;
}

function stopped()
{
    return stop();
}

function numbered()
{
    return number();
}
-----
<?php

namespace ReturnContract;

function stop(): never
{
    throw new \RuntimeException('stop');
}

function number(): int
{
    return 42;
}

function stopped()
{
    return stop();
}

function numbered(): int
{
    return number();
}
