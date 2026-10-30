<?php
// Pun leaves the class alone: Rector declares the constructor parameter `$application` twice (a fatal error).
namespace N;
use Illuminate\Support\Facades\App;
class A
{
    public function __construct(\Illuminate\Foundation\Application $application)
    {
        App::get(1);
    }
}
