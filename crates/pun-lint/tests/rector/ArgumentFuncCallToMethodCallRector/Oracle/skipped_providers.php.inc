<?php
namespace N;
class A
{
    public function anything(): mixed {}
    public function go() { return view('a'); }
}
-----
<?php
namespace N;
class A
{
    public function __construct(private readonly \Illuminate\Contracts\View\Factory $factory)
    {
    }
    public function anything(): mixed {}
    public function go() { return $this->factory->make('a'); }
}
