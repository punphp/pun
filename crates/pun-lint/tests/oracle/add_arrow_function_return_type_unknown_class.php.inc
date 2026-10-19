<?php

use Vendor\PendingDispatch;

function go(): ?PendingDispatch
{
    return null;
}

$a = fn () => go()->job;
$b = fn () => (new PendingDispatch())->job;
$c = fn () => go()?->job();
-----
<?php

use Vendor\PendingDispatch;

function go(): ?PendingDispatch
{
    return null;
}

$a = fn () => go()->job;
$b = fn () => (new PendingDispatch())->job;
$c = fn () => go()?->job();
