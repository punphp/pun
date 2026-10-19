<?php

namespace App;

class Thing
{
}

function prop()
{
    $thing = new Thing();
    $thing->name = 'x';

    return $thing;
}

function offset()
{
    $thing = new Thing();
    $thing['offset'] = 1;

    return $thing;
}

function nestedProp()
{
    $thing = new Thing();
    $thing->a->b = 1;

    return $thing;
}

function append()
{
    $thing = new Thing();
    $thing[] = 1;

    return $thing;
}

function compound()
{
    $thing = new Thing();
    $thing .= 'x';

    return $thing;
}

function unsetOffset()
{
    $thing = new Thing();
    unset($thing['a']);

    return $thing;
}
-----
<?php

namespace App;

class Thing
{
}

function prop(): \App\Thing
{
    $thing = new Thing();
    $thing->name = 'x';

    return $thing;
}

function offset()
{
    $thing = new Thing();
    $thing['offset'] = 1;

    return $thing;
}

function nestedProp(): \App\Thing
{
    $thing = new Thing();
    $thing->a->b = 1;

    return $thing;
}

function append()
{
    $thing = new Thing();
    $thing[] = 1;

    return $thing;
}

function compound()
{
    $thing = new Thing();
    $thing .= 'x';

    return $thing;
}

function unsetOffset()
{
    $thing = new Thing();
    unset($thing['a']);

    return $thing;
}
