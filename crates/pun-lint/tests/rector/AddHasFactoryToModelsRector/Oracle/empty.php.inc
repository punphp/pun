<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/** Doc. */
#[\Attribute]
final class A extends Model implements \JsonSerializable,   \Countable {}

#[\AllowDynamicProperties] #[\Deprecated]
readonly class B extends Model {}

class C extends Model { /* c */ }

class D extends Model
{
}

$x = new class(1, 2) extends Model implements \Countable {};
$y = new #[\AllowDynamicProperties] class extends Model {};

if (true) {
    class E extends Model {}
}

class   F   extends   Model   {}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

/** Doc. */
#[\Attribute]
final class A extends Model implements \JsonSerializable, \Countable
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}

#[\AllowDynamicProperties]
#[\Deprecated]
readonly class B extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}

class C extends Model { use \Illuminate\Database\Eloquent\Factories\HasFactory;
/* c */ }

class D extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}

$x = new class(1, 2) extends Model implements \Countable
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
};
$y = new #[\AllowDynamicProperties]
class extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
};

if (true) {
    class E extends Model
    {
        use \Illuminate\Database\Eloquent\Factories\HasFactory;
    }
}

class F extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}
