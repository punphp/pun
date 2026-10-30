<?php

namespace App;

use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Session\Session;
use Illuminate\Contracts\Session\Session as Sess;
use Illuminate\Support\Facades\Application;

function foo(Container $app, ?Container $maybe, \Illuminate\Container\Container $c, $x)
{
    $app->make(Session::class);
    $app->make(Sess::class);
    $app->get(Session::FOO);
    $app->MAKE("encrypter");
    $app->make('encrypter', ['a' => 1]);
    $app->make(abstract: 'encrypter');
    $app->make(...['encrypter']);
    $app->make('Encrypter');
    $app->make("encr\x79pter");
    $app->make(<<<'T'
    encrypter
    T);
    $maybe->make('encrypter');
    $c->make('encrypter');
    $x->make('encrypter');
    $app?->make('encrypter');
    $app->makeWith('encrypter');
    Application::get('encrypter');
    \App::make('encrypter');
    app('encrypter');
    \app('encrypter');
    resolve('encrypter');
    App('encrypter');
    app();
    app((Session::class));
    app('session');
    app(Session::class . '');
    app($x::class);
    app(static::class);
}
-----
<?php

namespace App;

use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Session\Session;
use Illuminate\Contracts\Session\Session as Sess;
use Illuminate\Support\Facades\Application;

function foo(Container $app, ?Container $maybe, \Illuminate\Container\Container $c, $x)
{
    $app->make('store');
    $app->make('store');
    $app->get('store');
    $app->MAKE(\Illuminate\Contracts\Encryption\Encrypter::class);
    $app->make(\Illuminate\Contracts\Encryption\Encrypter::class, ['a' => 1]);
    $app->make(abstract: \Illuminate\Contracts\Encryption\Encrypter::class);
    $app->make(...['encrypter']);
    $app->make('Encrypter');
    $app->make(\Illuminate\Contracts\Encryption\Encrypter::class);
    $app->make(\Illuminate\Contracts\Encryption\Encrypter::class);
    $maybe->make(\Illuminate\Contracts\Encryption\Encrypter::class);
    $c->make(\Illuminate\Contracts\Encryption\Encrypter::class);
    $x->make('encrypter');
    $app?->make('encrypter');
    $app->makeWith('encrypter');
    Application::get(\Illuminate\Contracts\Encryption\Encrypter::class);
    \App::make('encrypter');
    app(\Illuminate\Contracts\Encryption\Encrypter::class);
    \app(\Illuminate\Contracts\Encryption\Encrypter::class);
    resolve(\Illuminate\Contracts\Encryption\Encrypter::class);
    App(\Illuminate\Contracts\Encryption\Encrypter::class);
    app();
    app(('store'));
    app('store');
    app(Session::class . '');
    app($x::class);
    app(static::class);
}
