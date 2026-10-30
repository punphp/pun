<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model as Eloquent;

abstract class Base extends Eloquent {}

trait T
{
    protected $fillable = ['a'];
}

class User extends Base
{
    protected $fillable = ['a'];
}

class Plain
{
    use T;
}

class N extends \Illuminate\Database\Eloquent\Model
{
    protected $fillable = ['a'];
}

interface I {}

enum E
{
    const A = 1;
}
-----
<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model as Eloquent;

abstract class Base extends Eloquent {}

trait T
{
    protected $fillable = ['a'];
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class User extends Base
{
}

class Plain
{
    use T;
}

#[\Illuminate\Database\Eloquent\Attributes\Fillable(['a'])]
class N extends \Illuminate\Database\Eloquent\Model
{
}

interface I {}

enum E
{
    const A = 1;
}
