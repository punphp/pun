<?php
// Pun leaves the class alone: Rector reads the static property `$s` as `$this->s`, an undefined instance property.
namespace N;
use Illuminate\Contracts\View\Factory;
class D
{
    /** @var Factory|null */
    public static $s;
    public function go() { return view('a'); }
}
