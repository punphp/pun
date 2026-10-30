<?php
namespace N;
use Illuminate\Support\Facades\App;
class A extends B
{
    public function go() { return App::get(1); }
}
class B extends A
{
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
class A extends B
{
    public function go() { return App::get(1); }
}
class B extends A
{
}
