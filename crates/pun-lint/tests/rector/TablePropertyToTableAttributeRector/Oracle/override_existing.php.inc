<?php

use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Model;

#[Table(name: 'a', key: 'k', keyType: 'int')]
class A extends Model
{
    protected $keyType = 'string';
    protected $primaryKey = 'uid';
}
-----
<?php

use Illuminate\Database\Eloquent\Attributes\Table;
use Illuminate\Database\Eloquent\Model;

#[\Illuminate\Database\Eloquent\Attributes\Table(name: 'a', key: 'uid', keyType: 'string')]
class A extends Model
{
}
