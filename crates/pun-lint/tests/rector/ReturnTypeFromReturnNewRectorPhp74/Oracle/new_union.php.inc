<?php

namespace App;

class One
{
}

class Two
{
}

function either($x)
{
    if ($x) {
        return new One();
    }

    return new Two();
}
