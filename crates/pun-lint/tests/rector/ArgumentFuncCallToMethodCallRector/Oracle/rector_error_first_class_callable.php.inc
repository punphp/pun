<?php
namespace N;
class A
{
    public function go() { return view('a'); }
    public function callable() { return session(...); }
}
-----
<?php
namespace N;
class A
{
    public function go() { return view('a'); }
    public function callable() { return session(...); }
}
