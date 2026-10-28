<?php

namespace App\Http\Controllers;

class PostController
{
    public function __construct()
    {
        $this->MIDDLEWARE('auth')->only('store');
        $This->middleware('guest');
        $other->middleware('auth');
        $this?->middleware('auth');
        $this->{'middleware'}('auth');
        $f = function () {
            $this->middleware('closure');
        };
        $c = new class {
            public function run()
            {
                $this->middleware('anonymous');
            }
        };
    }
}

class Postcontroller
{
    public function run()
    {
        $this->middleware('case');
    }
}

trait ThingController
{
    public function run()
    {
        $this->middleware('trait');
    }
}

class Plain
{
    public function run()
    {
        $this->middleware('plain');
    }
}

$this->middleware('top');
