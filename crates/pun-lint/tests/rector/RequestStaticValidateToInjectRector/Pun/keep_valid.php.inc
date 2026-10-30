<?php

// Rector replaces `request()` inside a closure, a nested function and an anonymous class's method with the method's
// `$request`, which is not defined there; adds a second `$request` next to an untyped one (a fatal error); calls
// `->input()` on a variadic request parameter, which holds an array; and adds a parameter after a variadic one (a
// fatal error). Pun leaves those calls and methods; it changes the anonymous class's method on its own.

namespace App;

use Illuminate\Http\Request;

class Controller
{
    public function g()
    {
        return function () {
            return request('x');
        };
    }

    public function m($request)
    {
        return request('x');
    }

    public function p(Request ...$all)
    {
        return request('x');
    }

    public function r()
    {
        $x = new class {
            public function s()
            {
                return request('x');
            }
        };
        return request(...['a']);
    }

    public function y(int ...$ids)
    {
        return request('x');
    }

    public function z()
    {
        function inner()
        {
            return request('x');
        }
    }
}
-----
<?php

// Rector replaces `request()` inside a closure, a nested function and an anonymous class's method with the method's
// `$request`, which is not defined there; adds a second `$request` next to an untyped one (a fatal error); calls
// `->input()` on a variadic request parameter, which holds an array; and adds a parameter after a variadic one (a
// fatal error). Pun leaves those calls and methods; it changes the anonymous class's method on its own.

namespace App;

use Illuminate\Http\Request;

class Controller
{
    public function g()
    {
        return function () {
            return request('x');
        };
    }

    public function m($request)
    {
        return request('x');
    }

    public function p(Request ...$all)
    {
        return request('x');
    }

    public function r(\Illuminate\Http\Request $request)
    {
        $x = new class {
            public function s(\Illuminate\Http\Request $request)
            {
                return $request->input('x');
            }
        };
        return $request->input(...['a']);
    }

    public function y(int ...$ids)
    {
        return request('x');
    }

    public function z()
    {
        function inner()
        {
            return request('x');
        }
    }
}
