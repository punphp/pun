<?php
class P
{
    public function __construct(int $a)
    {
    }
}
class A extends P
{
    public function go() { return \Illuminate\Support\Facades\App::get(1); }
}
-----
<?php
class P
{
    public function __construct(int $a)
    {
    }
}
class A extends P
{
    public function __construct(int $a, private \Illuminate\Foundation\Application $application)
    {
        parent::__construct($a);
    }
    public function go() { return $this->application->get(1); }
}
