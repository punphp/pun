<?php

namespace App;

use Illuminate\Http\Request;

class Controller
{
    public function store()
    {
        $request = Request::create(
            '/resource',
            'POST',
            [
                'a' => 'b',
            ],
        );
        return request(
            'x',
            [
                'y',
            ]
        );
    }
}
-----
<?php

namespace App;

use Illuminate\Http\Request;

class Controller
{
    public function store(\Illuminate\Http\Request $request)
    {
        $request = Request::create(
            '/resource',
            'POST',
            [
                'a' => 'b',
            ],
        );
        return $request->input('x', [
            'y',
        ]);
    }
}
