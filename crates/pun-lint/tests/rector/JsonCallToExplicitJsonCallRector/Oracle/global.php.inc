<?php

use Illuminate\Foundation\Testing\Concerns\MakesHttpRequests;

class GlobalUsesTrait
{
    use MakesHttpRequests;

    public function test()
    {
        $this->json('get', '/');
    }
}
-----
<?php

use Illuminate\Foundation\Testing\Concerns\MakesHttpRequests;

class GlobalUsesTrait
{
    use MakesHttpRequests;

    public function test()
    {
        $this->getJson('/');
    }
}
