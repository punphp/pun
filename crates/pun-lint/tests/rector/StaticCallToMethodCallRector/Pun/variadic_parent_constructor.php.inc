<?php
// Pun leaves the class alone: Rector adds a parameter after the variadic one (a fatal error) and passes the variadic as one argument.
namespace N;
use Illuminate\Support\Facades\App;
class PVar
{
    public function __construct(string ...$names) {}
}
class A extends PVar
{
    public function go() { return App::get('y'); }
}
