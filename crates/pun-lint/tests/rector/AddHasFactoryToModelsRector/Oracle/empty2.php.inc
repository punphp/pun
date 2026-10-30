<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

function f($a)
{
    return new class(
        $a,
        [
            1,
        ]
    ) extends Model {};
}

if (true) {
    #[\AllowDynamicProperties]
    abstract CLASS E extends Model implements \Countable
    {
    }
    $z = new readonly class() extends Model {};
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

function f($a)
{
    return new class($a, [
        1,
    ]) extends Model
    {
        use \Illuminate\Database\Eloquent\Factories\HasFactory;
    };
}

if (true) {
    #[\AllowDynamicProperties]
    abstract class E extends Model implements \Countable
    {
        use \Illuminate\Database\Eloquent\Factories\HasFactory;
    }
    $z = new readonly class extends Model
    {
        use \Illuminate\Database\Eloquent\Factories\HasFactory;
    };
}
