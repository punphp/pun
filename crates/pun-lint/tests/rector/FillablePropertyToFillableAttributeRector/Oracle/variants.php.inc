<?php

use Illuminate\Database\Eloquent\Model;

$a = new class extends Model {
    protected $fillable = ['a'];
};

$b = new class() extends Model implements \Countable { protected $fillable = ['a']; };

class B extends Model { protected $fillable = ['a']; }

class C extends Model
{
    protected $fillable = array('a', 'b');
    protected $guarded = ['x'];
}

abstract class D extends Model
{
    protected static $fillable = ['a'];
}

class E extends Model
{
    protected array $fillable = ['a', ...['b']];
}

class F extends Model
{
    protected array $fillable = ['k' => 'a', 'l' => "b"];
}

class G extends Model
{
    protected $fillable = ['a', 'b' . 'c'];
}

class H extends \Illuminate\Foundation\Auth\Unknown
{
    protected $fillable = ['a'];
}

class I extends Model
{
    protected $fillable;
    protected $other = 1, $fillable = ['a'];
}

class J extends Model
{
    private $fillable = ['a'];
}

class K extends Model
{
    protected $other = 1, $fillable = ['a'];
}

class Base extends Model {}

final class L extends Base
{
    protected $fillable = [('a'), <<<X
    b
    X, <<<'Y'
    c
    Y];
}

class M extends Model
{
    protected $fillable = ["a$b"];
    protected $Fillable = ['a'];
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

$a = new #[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class extends Model
{
};

$b = new #[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class extends Model implements \Countable
{
};

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class B extends Model
{
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(array('a', 'b'))]
class C extends Model
{
    protected $guarded = ['x'];
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
abstract class D extends Model
{
}

class E extends Model
{
    protected array $fillable = ['a', ...['b']];
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['k' => 'a', 'l' => "b"])]
class F extends Model
{
}

class G extends Model
{
    protected $fillable = ['a', 'b' . 'c'];
}

class H extends \Illuminate\Foundation\Auth\Unknown
{
    protected $fillable = ['a'];
}

class I extends Model
{
    protected $fillable;
    protected $other = 1, $fillable = ['a'];
}

class J extends Model
{
    private $fillable = ['a'];
}

class K extends Model
{
    protected $other = 1, $fillable = ['a'];
}

class Base extends Model {}

#[\Illuminate\Database\Eloquent\Attributes\Fillable([('a'), <<<X
    b
    X, <<<'Y'
    c
    Y])]
final class L extends Base
{
}

class M extends Model
{
    protected $fillable = ["a$b"];
    protected $Fillable = ['a'];
}
