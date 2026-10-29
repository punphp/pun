<?php
// Pun leaves the class alone: the constructor comes from a trait; Rector calls `parent::__construct()` in a class without a parent (a fatal error).
namespace N;
use Illuminate\Support\Facades\App;
trait WithCtor { public function __construct(public int $v = 1) { echo 1; } }
class A
{
    use WithCtor;
    public function go() { return App::get('y'); }
}
