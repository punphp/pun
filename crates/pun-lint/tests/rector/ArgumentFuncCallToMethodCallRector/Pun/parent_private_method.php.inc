<?php
// Pun leaves the class alone: Rector calls the parent's private method `hidden()`, which the child cannot call.
namespace N;
class PrivMethod
{
    private function hidden(): \Illuminate\Contracts\View\Factory {}
}
class A extends PrivMethod
{
    public function go() { return view('a'); }
}
