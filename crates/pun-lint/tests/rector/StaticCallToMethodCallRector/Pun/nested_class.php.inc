<?php
// Pun leaves the class alone: Rector rewrites the call in the nested class to `$this->application`, a property the nested class does not have.
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function go() {
        return new class { public function in() { return App::get('z'); } };
    }
}
