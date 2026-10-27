<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/**
 * Doc of the class.
 */
#[Foo]
final class User extends Model
{
    use HasFactory;

    // a comment
    /** @var list<string> */
    protected $fillable = [
        'name', // name
        "email",
    ], $other = 1;

    public function x() {}
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/**
 * Doc of the class.
 */
#[Foo]
#[\Illuminate\Database\Eloquent\Attributes\Fillable([
    'name', // name
    "email",
])]
final class User extends Model
{
    use HasFactory;

    public function x() {}
}
