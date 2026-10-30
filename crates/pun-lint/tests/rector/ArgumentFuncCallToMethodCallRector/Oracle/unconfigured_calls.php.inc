<?php
namespace N;
class A
{
    public function go($f) { return $f('a') . strlen('b') . view('c'); }
}
-----
<?php
namespace N;
class A
{
    public function __construct(private readonly \Illuminate\Contracts\View\Factory $factory)
    {
    }
    public function go($f) { return $f('a') . strlen('b') . $this->factory->make('c'); }
}
