<?php
namespace N;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\Response;
class P
{
    public function __construct(public $responseFactory)
    {
    }
}
class A extends P
{
    public function go() { return App::get(1) . Response::make(2) . App::make(3); }
}
-----
<?php
namespace N;
use Illuminate\Support\Facades\App;
use Illuminate\Support\Facades\Response;
class P
{
    public function __construct(public $responseFactory)
    {
    }
}
class A extends P
{
    public function __construct(public $responseFactory, private \Illuminate\Foundation\Application $application)
    {
        parent::__construct($responseFactory);
    }
    public function go() { return $this->application->get(1) . $this->responseFactory->make(2) . $this->application->make(3); }
}
