<?php

use Illuminate\Support\Traits\ReflectsClosures;

if (! function_exists('lazy')) {
    /**
     * Create a lazy instance.
     *
     * @template TValue of object
     *
     * @param  class-string<TValue>|(\Closure(TValue): mixed)  $class
     * @param  (\Closure(TValue): mixed)|int  $callback
     * @param  int  $options
     * @param  array<string, mixed>  $eager
     * @return TValue
     *
     * @throws \ReflectionException
     */
    function lazy($class, $callback = 0, $options = 0, $eager = [])
    {}
}

if (! function_exists('proxy')) {
    /**
     * Create a lazy proxy instance.
     *
     * @template TValue of object
     *
     * @param  class-string<TValue>|(\Closure(TValue): TValue)  $class
     * @param  (\Closure(TValue): TValue)|int  $callback
     * @param  int  $options
     * @param  array<string, mixed>  $eager
     * @return TValue
     *
     * @throws \ReflectionException
     */
    function proxy($class, $callback = 0, $options = 0, $eager = [])
    {}
}
