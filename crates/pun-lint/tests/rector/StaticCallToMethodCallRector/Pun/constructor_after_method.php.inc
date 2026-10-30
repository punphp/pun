<?php
// Pun leaves the class alone: Rector declares the constructor parameter `$application` twice (a fatal error).
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go() { return App::get(1); }
    public function __construct()
    {
        App::make(2);
    }
}
