<?php

use Illuminate\Database\Eloquent\Model;

$a = new class extends Model {
    protected $fillable = ['a'];
    public function x() { return 1; }
};

#[Foo] final readonly class B extends Model implements \Countable,
    \Stringable { protected $fillable = ['a']; }

class C extends Model
{
    protected $fillable = ['a'];
    // trailing
}

/** doc */
#[Foo(1), Bar]
#[Baz]
abstract class D extends Model
{
    /** doc */
    protected $fillable = [
        'a',
    ];
}

class E extends Model
{
    protected $guarded = [];

    protected $fillable = ['a'];

    protected $hidden = [];
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

$a = new #[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])] class extends Model {
    public function x() { return 1; }
};

#[Foo]
#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
final readonly class B extends Model implements \Countable, \Stringable
{
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class C extends Model
{
    // trailing
}

/** doc */
#[Foo(1), Bar]
#[Baz]
#[\Illuminate\Database\Eloquent\Attributes\Fillable([
    'a',
])]
abstract class D extends Model
{
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class E extends Model
{
    protected $guarded = [];

    protected $hidden = [];
}
