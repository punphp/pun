<?php

namespace App\Tests;

use Illuminate\Foundation\Testing\Concerns\MakesHttpRequests;

class UsesTrait
{
    use MakesHttpRequests;

    public function test()
    {
        $this->json('get', '/');
    }
}

class Other
{
    public function test()
    {
        $this->json('get', '/');
    }
}

trait InTrait
{
    public function test()
    {
        $this->json('get', '/');
    }
}

enum InEnum
{
    use MakesHttpRequests;

    public function test()
    {
        $this->json('get', '/');
    }
}

$anonymous = new class {
    use MakesHttpRequests;

    public function test()
    {
        $this->json('get', '/');
    }
};

$this->json('get', '/');
-----
<?php

namespace App\Tests;

use Illuminate\Foundation\Testing\Concerns\MakesHttpRequests;

class UsesTrait
{
    use MakesHttpRequests;

    public function test()
    {
        $this->getJson('/');
    }
}

class Other
{
    public function test()
    {
        $this->json('get', '/');
    }
}

trait InTrait
{
    public function test()
    {
        $this->json('get', '/');
    }
}

enum InEnum
{
    use MakesHttpRequests;

    public function test()
    {
        $this->getJson('/');
    }
}

$anonymous = new class {
    use MakesHttpRequests;

    public function test()
    {
        $this->getJson('/');
    }
};

$this->json('get', '/');
