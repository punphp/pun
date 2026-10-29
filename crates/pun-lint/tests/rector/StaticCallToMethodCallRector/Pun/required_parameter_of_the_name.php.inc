<?php
// Pun leaves the class alone: Rector declares the property but never assigns it, the `@required` method having a parameter of its name, so `$this->application` reads an uninitialized property.
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    /** @required */
    public function autowire($application)
    {
        echo 1;
    }
    public function go() { return App::get(1); }
}
