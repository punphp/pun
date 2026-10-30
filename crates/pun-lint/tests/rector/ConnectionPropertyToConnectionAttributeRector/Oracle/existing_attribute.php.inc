<?php

use Illuminate\Database\Eloquent\Attributes\Connection;
use Illuminate\Database\Eloquent\Model;

#[Connection('mysql')]
class A extends Model
{
    protected $connection = 'sqlite';
}

class B extends Model
{
    protected $connection = 1;
}
-----
<?php

use Illuminate\Database\Eloquent\Attributes\Connection;
use Illuminate\Database\Eloquent\Model;

#[Connection('mysql')]
#[\Illuminate\Database\Eloquent\Attributes\Connection('sqlite')]
class A extends Model
{
}

class B extends Model
{
    protected $connection = 1;
}
