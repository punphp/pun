<?php
// Pun leaves the class alone: Rector declares the constructor parameter `$application` twice (a fatal error) and reads `$this->application` in go(), which no class declares.
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct()
    {
        App::get('a');
        App::get('b');
    }
    public function go() { return App::get('y'); }
}
