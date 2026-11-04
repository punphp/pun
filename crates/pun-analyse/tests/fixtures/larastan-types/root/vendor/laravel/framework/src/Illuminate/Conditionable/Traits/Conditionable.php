<?php

namespace Illuminate\Support\Traits;

use Closure;
use Illuminate\Support\HigherOrderWhenProxy;

trait Conditionable
{
    /**
     * Apply the callback if the given "value" is (or resolves to) truthy.
     *
     * @template TWhenParameter
     * @template TWhenReturnType
     *
     * @param  (\Closure($this): TWhenParameter)|TWhenParameter|null  $value
     * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $callback
     * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $default
     * @return $this|TWhenReturnType
     */
    public function when($value = null, ?callable $callback = null, ?callable $default = null)
    { func_get_args(); }

    /**
     * Apply the callback if the given "value" is (or resolves to) falsy.
     *
     * @template TUnlessParameter
     * @template TUnlessReturnType
     *
     * @param  (\Closure($this): TUnlessParameter)|TUnlessParameter|null  $value
     * @param  (callable($this, TUnlessParameter): TUnlessReturnType)|null  $callback
     * @param  (callable($this, TUnlessParameter): TUnlessReturnType)|null  $default
     * @return $this|TUnlessReturnType
     */
    public function unless($value = null, ?callable $callback = null, ?callable $default = null)
    { func_get_args(); }
}
