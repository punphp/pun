<?php

namespace App;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class A extends Model {}

class B extends Model { }

class C extends Model
{
    protected $table = 'c';

    public function x() {}
}

class D extends Model
{
    use SomeTrait;
}

trait SomeTrait
{
    use HasFactory;
}

class E extends Model
{
    use OtherTrait;
}

trait OtherTrait {}

class Base extends Model
{
    use HasFactory;
}

class F extends Base {}

abstract class G extends Model
{
    // a comment
}

final class H extends Model
{
    /** doc */
    public $x;
}

class I
{
}

new class extends Model {};

class J extends \Illuminate\Database\Eloquent\Model
{
    use \Illuminate\Database\Eloquent\SoftDeletes, OtherTrait;
}

class K extends Missing {}

    class L extends Model
    {
        public $y;
    }
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;

class A extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}

class B extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}

class C extends Model
{
    protected $table = 'c';

    public function x() {}
}

class D extends Model
{
    use SomeTrait;
}

trait SomeTrait
{
    use HasFactory;
}

class E extends Model
{
    use OtherTrait;
}

trait OtherTrait {}

class Base extends Model
{
    use HasFactory;
}

class F extends Base {}

abstract class G extends Model
{
    // a comment
}

final class H extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
    /** doc */
    public $x;
}

class I
{
}

new class extends Model {};

class J extends \Illuminate\Database\Eloquent\Model
{
    use \Illuminate\Database\Eloquent\SoftDeletes, OtherTrait;
}

class K extends Missing {}

    class L extends Model
    {
        public $y;
    }
