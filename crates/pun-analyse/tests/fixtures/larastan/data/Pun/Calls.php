<?php

namespace Pun\Calls;

use App\User;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Auth;

enum Suit
{
    case Hearts;
}

class Invokable
{
    public function __invoke()
    {
    }
}

class Plain
{
    public static function make()
    {
    }

    public function run()
    {
        return $this;
    }
}

function helpers(Plain $plain, string $s, $m)
{
    with(1, 2);
    with(1, 1.5);
    with(1, true);
    with(1, null);
    with(1, 'strlen');
    with(1, 'pun_no_such_function');
    with(1, '\Pun\Calls\Plain::run');
    with(1, 'Pun\Calls\Plain::nope');
    with(1, 'Pun\Calls\Missing::run');
    with(1, Plain::class);
    with(1, new Invokable());
    with(1, $plain);
    with(1, Suit::Hearts);
    with(1, fn () => 1);
    with(1, $s);
    with(1, $m);
    value(null);
    value(1);
    value([1]);
    value(new Plain());
    value(Suit::Hearts);
    value(fn () => 1);
    value(function () {
    });
    value($m);
    value();
    with();
    $f = 'with';
    $f(1);
}

function makes(User $user, Plain $plain, $m)
{
    Plain::make();
    $user::make();
    $plain::make();
    $class = User::class;
    $class::make();
    $name = 'App\User';
    $name::make();
    $other = 'Pun\Calls\Plain';
    $other::make();
    $text = 'not a class';
    $text::make();
    Plain::$m();
    User::$m();
}

function facade(Request $request, $m)
{
    Auth::id();
    Auth::$m();
    Auth::check();
    Auth::guest();
    $request->$m();
    auth('web')->user();
    $request->user()->user();
    auth()->check();
    auth()->guest();
    $f = 'auth';
    $f()->user();
}

function nullable(?Request $request)
{
    Auth::user();
    auth()->user();
}

function defaulted(Request $request = null)
{
    Auth::user();
}

function untyped($request)
{
    Auth::user();
}

function nested(Request $request)
{
    $inner = function () {
        Auth::user();
    };
    $used = function () use ($request) {
        Auth::user();
    };
    $typed = fn (Request $request) => Auth::user();
}
