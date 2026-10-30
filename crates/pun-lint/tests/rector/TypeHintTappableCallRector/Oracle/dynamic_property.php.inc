<?php

namespace App;

class Box
{
    public Box $known;
}

function run(Box $box, string $name)
{
    // a property named by an expression keeps Pun's reading of the value's type
    tap($box->$name, function (Box $x) {
    });
    tap($box->{'known'}, function (Box $x) {
    });
}
-----
<?php

namespace App;

class Box
{
    public Box $known;
}

function run(Box $box, string $name)
{
    // a property named by an expression keeps Pun's reading of the value's type
    tap($box->$name, function ($x) {
    });
    tap($box->{'known'}, function (Box $x) {
    });
}
