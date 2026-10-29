<?php
// Pun leaves the class alone: Rector reads `$this->application` in a function, where there is no `$this`.
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go()
    {
        function helper() { return App::get(1); }
    }
}
