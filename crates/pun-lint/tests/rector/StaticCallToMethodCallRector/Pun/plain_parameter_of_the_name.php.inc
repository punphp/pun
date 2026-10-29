<?php
// Pun leaves the class alone: Rector keeps the plain parameter `$application` and reads `$this->application`, which no class declares.
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct($application)
    {
    }
    public function go() { return App::get(1); }
}
