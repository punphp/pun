<?php

namespace Pun\Octane;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Support\ServiceProvider;

class Service
{
    public function __construct($app = null)
    {
    }

    public function singleton($abstract, $concrete = null)
    {
    }
}

class Provider extends ServiceProvider
{
    public function register($m = null, $mixed = null, Application|Service|null $either = null)
    {
        $this->app->singleton('a', (function () {
            return $this->app;
        }));
        $this->app->$m('b', function () {
            return $this->app;
        });
        $this->app->singleton('c', 'd');
        $this->app->singleton('e');
        $mixed->singleton('f', function () {
            return $this->app;
        });
        $either->singleton('g', function () {
            return $this->app;
        });
        (new Service())->singleton('h', function () {
            return $this->app;
        });
        $this->app->singleton('i', function ($app) {
            new Service();
            new Service(1);
            new Service($app['cache']);
            new Service($other['config']);
            new Service($other);
            return new Service($app['config']);
        });
        $this->app->bind('j', function () {
            return $this->app->make('x');
        });
        $this->app->bind('k', function ($app) {
            return new Service($app);
        });
        $this->app->singleton('l', function () {
            return $this->other;
        });
    }
}
