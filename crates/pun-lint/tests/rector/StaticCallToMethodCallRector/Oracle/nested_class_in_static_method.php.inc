<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public static function make() {
        return new class { public function in() { return App::get('z'); } };
    }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public static function make() {
        return new class { public function in() { return (new \Illuminate\Foundation\Application())->get('z'); } };
    }
}
