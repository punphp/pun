<?php
// Pun leaves the class alone: the parent takes `$application` as a plain parameter; Rector adds no dependency and reads `$this->application`, which no class declares.
namespace N;
use Illuminate\Support\Facades\App;
class WithPlain
{
    public function __construct($application) {}
}
class A extends WithPlain
{
    public function go() { return App::get('y'); }
}
