<?php
namespace N;
class A
{
    public function go() { return view('a'); }
    public function guard() { return auth('web'); }
}
-----
<?php
namespace N;
class A
{
    public function go() { return view('a'); }
    public function guard() { return auth('web'); }
}
