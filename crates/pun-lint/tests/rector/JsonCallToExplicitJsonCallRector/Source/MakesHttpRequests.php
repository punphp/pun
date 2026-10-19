<?php

namespace Illuminate\Foundation\Testing\Concerns;

trait MakesHttpRequests
{
    public function json($method, $uri, array $data = [], array $headers = [], $options = 0)
    {
    }
}
