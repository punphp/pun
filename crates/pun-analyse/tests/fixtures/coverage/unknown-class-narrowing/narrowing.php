<?php

namespace InterfaceNarrowing;

class Router
{
    /** @param mixed $response */
    public static function toResponse($response): mixed
    {
        if ($response instanceof PsrResponse) {
            $response = 1;
        } elseif ($response instanceof Model) {
            $response = 2;
        } elseif ($response instanceof Stringable) {
            $response = 3;
        } elseif (! $response instanceof SymfonyResponse && ($response instanceof Arrayable || $response instanceof Jsonable || is_array($response))) {
            $response = 4;
        }

        return $response;
    }
}
