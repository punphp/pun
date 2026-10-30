<?php
// Pun leaves the class alone: Rector gives the abstract `@required` method a body (a fatal error).
namespace N;
use Illuminate\Support\Facades\App;
abstract class A
{
    /** @required */
    abstract public function autowire();
    public function go() { return App::get(1); }
}
