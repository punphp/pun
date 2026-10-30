<?php

use Illuminate\Database\Eloquent\Model;

class A extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;

    const X = 1;

    protected $table = 'a';

    public function newCollection(array $models = [])
    {
        return new Users($models);
    }
}

class B extends Model
{
    public function newCollection(array $models = [])
    {
        throw new \LogicException();
    }
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

#[\Illuminate\Database\Eloquent\Attributes\CollectedBy(Users::class)]
class A extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;

    const X = 1;

    protected $table = 'a';
}

class B extends Model
{
    public function newCollection(array $models = [])
    {
        throw new \LogicException();
    }
}
