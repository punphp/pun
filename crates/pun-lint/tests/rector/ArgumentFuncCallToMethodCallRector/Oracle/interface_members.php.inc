<?php
namespace N;
use Illuminate\Contracts\View\Factory;
interface HasViews
{
    public Factory $views { get; }
}
interface MakesViews
{
    public function views(): Factory;
}
abstract class A implements HasViews
{
    public function go() { return view('a'); }
}
abstract class B implements MakesViews
{
    public function go() { return view('a'); }
}
-----
<?php
namespace N;
use Illuminate\Contracts\View\Factory;
interface HasViews
{
    public Factory $views { get; }
}
interface MakesViews
{
    public function views(): Factory;
}
abstract class A implements HasViews
{
    public function go() { return $this->views->make('a'); }
}
abstract class B implements MakesViews
{
    public function go() { return $this->views()->make('a'); }
}
