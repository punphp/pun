<?php
// Pun leaves the class alone: Rector passes the dependency as a constructor parameter, then reads `$application` in a closure that does not capture it.
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct()
    {
        $f = function () { return App::get(1); };
    }
}
