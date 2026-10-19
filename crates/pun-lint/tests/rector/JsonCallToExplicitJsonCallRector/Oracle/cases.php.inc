<?php

namespace App;

use Illuminate\Foundation\Testing\Concerns\MakesHttpRequests;

class Cases
{
    public function run(MakesHttpRequests $http, $verb, $data)
    {
        $http->json('GET', route('home'), $data, []);
        $http->json('get', '/x', $data);
        $http->json('PATCH', '/', ['x'], ['h']);
        $http->json($verb, '/');
        $http->json("post", '/');
        $http->JSON('post', '/');
        $http->json(
            'delete',
            '/x',
        );
        $http->json('OPTIONS', '/');
        $other->json('post', '/');
        $http->json('get', route('home'), data: $data);
    }
}
-----
<?php

namespace App;

use Illuminate\Foundation\Testing\Concerns\MakesHttpRequests;

class Cases
{
    public function run(MakesHttpRequests $http, $verb, $data)
    {
        $http->json('GET', route('home'), $data, []);
        $http->json('get', '/x', $data);
        $http->patchJson('/', ['x'], ['h']);
        $http->json($verb, '/');
        $http->postJson('/');
        $http->JSON('post', '/');
        $http->deleteJson(
            '/x',
        );
        $http->optionsJson('/');
        $other->json('post', '/');
        $http->getJson(route('home', $data));
    }
}
