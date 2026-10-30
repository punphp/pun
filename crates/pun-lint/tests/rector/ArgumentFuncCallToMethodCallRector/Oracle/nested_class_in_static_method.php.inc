<?php
namespace N;
class A
{
    public static function make() {
        return new class { public function in() { return view('z'); } };
    }
}
-----
<?php
namespace N;
class A
{
    public static function make() {
        return new class { public function __construct(private readonly \Illuminate\Contracts\View\Factory $factory)
        {
        }
        public function in() { return $this->factory->make('z'); } };
    }
}
