<?php
// Pun leaves the class alone: Rector gives the abstract constructor a body (a fatal error).
namespace N;
use Illuminate\Support\Facades\App;
abstract class A
{
    abstract public function __construct();
    public function go() { return App::get(1); }
}
