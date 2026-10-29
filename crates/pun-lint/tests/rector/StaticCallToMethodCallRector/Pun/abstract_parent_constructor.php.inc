<?php
// Pun leaves the class alone: Rector copies the abstract parent constructor with a body, still abstract (a fatal error).
namespace N;
use Illuminate\Support\Facades\App;
abstract class P
{
    abstract public function __construct(int $a);
}
abstract class A extends P
{
    public function go() { return App::get(1); }
}
