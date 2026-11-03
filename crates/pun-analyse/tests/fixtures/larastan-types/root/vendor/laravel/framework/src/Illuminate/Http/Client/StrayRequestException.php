<?php

namespace Illuminate\Http\Client;

use RuntimeException;

class StrayRequestException extends RuntimeException
{
    public function __construct(string $uri)
    {}
}
