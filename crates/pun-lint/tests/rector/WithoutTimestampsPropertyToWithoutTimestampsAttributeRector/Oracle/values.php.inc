<?php

use Illuminate\Database\Eloquent\Model;

class A extends Model
{
    public $timestamps = FALSE;
}

class B extends Model
{
    public $timestamps = \false;
}

class C extends Model
{
    public $timestamps = 0;
}

class D extends Model
{
    protected $timestamps = false;
}

class E extends Model
{
    var $timestamps = false;
}

class F extends Model
{
    public $timestamps = true;
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

#[\Illuminate\Database\Eloquent\Attributes\WithoutTimestamps]
class A extends Model
{
}

#[\Illuminate\Database\Eloquent\Attributes\WithoutTimestamps]
class B extends Model
{
}

class C extends Model
{
    public $timestamps = 0;
}

class D extends Model
{
    protected $timestamps = false;
}

#[\Illuminate\Database\Eloquent\Attributes\WithoutTimestamps]
class E extends Model
{
}

class F extends Model
{
    public $timestamps = true;
}
