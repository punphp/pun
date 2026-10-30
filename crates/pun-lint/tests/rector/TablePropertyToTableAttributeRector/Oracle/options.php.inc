<?php

use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Model;

#[Foo, Table(incrementing: false, name: 'old')]
class A extends Model
{
    protected $primaryKey = 'uid';
    protected $keyType = 'string';

    public function x() {}
}

#[Table(name: 1)]
class B extends Model
{
    protected $primaryKey = 'uid';
}

class C extends Model
{
    protected $table = 'c';
    protected $primaryKey;
    protected $keyType = 'int';
    public $incrementing = false;
}

#[Table(name: 'd', key: 'k')]
class D extends Model
{
    public $primaryKey = 'uid';
}

class E extends Model
{
    protected $primaryKey = 'uid';
}

#[Table('e')]
class F extends Model
{
    protected $primaryKey = 'uid';
}

class G extends Model
{
    protected $table = 'g', $primaryKey = 'uid';
}

class H extends Model
{
    protected $incrementing = !true;
    protected $table = 'h';
}

#[Table(name: 'i', key: 'a'), Table(keyType: 'string')]
class I extends Model
{
    protected $incrementing = false;
}
-----
<?php

use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Model;

#[\Illuminate\Database\Eloquent\Attributes\Table(name: 'old', incrementing: false, key: 'uid', keyType: 'string')]
class A extends Model
{
    public function x() {}
}

#[Table(name: 1)]
class B extends Model
{
    protected $primaryKey = 'uid';
}

#[\Illuminate\Database\Eloquent\Attributes\Table(name: 'c', keyType: 'int')]
class C extends Model
{
    protected $primaryKey;
    public $incrementing = false;
}

#[Table(name: 'd', key: 'k')]
class D extends Model
{
    public $primaryKey = 'uid';
}

class E extends Model
{
    protected $primaryKey = 'uid';
}

#[Table('e')]
class F extends Model
{
    protected $primaryKey = 'uid';
}

#[\Illuminate\Database\Eloquent\Attributes\Table(name: 'g', key: 'g')]
class G extends Model
{
}

#[\Illuminate\Database\Eloquent\Attributes\Table(name: 'h', incrementing: !true)]
class H extends Model
{
}

#[\Illuminate\Database\Eloquent\Attributes\Table(name: 'i', key: 'a', keyType: 'string', incrementing: false)]
class I extends Model
{
}
