<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

class A extends Model
{
    public function NewCollection(array $models = [])
    {
        return (new \App\Collections\Users($models));
    }

    public function other() {}
}

class B extends Model
{
    public function newCollection(array $models = [])
    {
        $x = 1;
        return new Users($models);
    }
}

class C extends Model
{
    public function newCollection(array $models = [])
    {
        return new Users($models);
        // done
    }
}

class D extends Model
{
    public function newCollection(array $models = [])
    {
        return new $this->collection($models);
    }
}

class E extends Model
{
    public function newCollection(array $models = [])
    {
        return parent::newCollection($models);
    }
}

class F extends Model
{
    public function newCollection(array $models = [])
    {
        return new self($models);
    }
}

abstract class G extends Model
{
    abstract public function newCollection(array $models = []);
}

class H extends Model
{
    public function newCollection(array $models = [])
    {
        return;
    }
}

class I extends Model
{
    public function newCollection(array $models = [])
    {
        return new class($models) {};
    }
}

class J extends Model
{
    public function newCollection(array $models = [])
    {
        return new parent($models);
    }
}

class K extends Model
{
}
-----
<?php

namespace App;

use Illuminate\Database\Eloquent\Model;

#[\Illuminate\Database\Eloquent\Attributes\CollectedBy(\App\Collections\Users::class)]
class A extends Model
{
    public function other() {}
}

class B extends Model
{
    public function newCollection(array $models = [])
    {
        $x = 1;
        return new Users($models);
    }
}

class C extends Model
{
    public function newCollection(array $models = [])
    {
        return new Users($models);
        // done
    }
}

class D extends Model
{
    public function newCollection(array $models = [])
    {
        return new $this->collection($models);
    }
}

class E extends Model
{
    public function newCollection(array $models = [])
    {
        return parent::newCollection($models);
    }
}

#[\Illuminate\Database\Eloquent\Attributes\CollectedBy(self::class)]
class F extends Model
{
}

abstract class G extends Model
{
    abstract public function newCollection(array $models = []);
}

class H extends Model
{
    public function newCollection(array $models = [])
    {
        return;
    }
}

class I extends Model
{
    public function newCollection(array $models = [])
    {
        return new class($models) {};
    }
}

#[\Illuminate\Database\Eloquent\Attributes\CollectedBy(parent::class)]
class J extends Model
{
}

class K extends Model
{
}
