<?php

namespace Pun\Oracle\SafeStrict;

function f(\DateTime $d): void
{
    $d->format(1);
}
