<?php

final class Rules
{
    /**
     * @param  array{0?: 'strict'}  $parameters
     */
    public function a($parameters)
    {
        return ($parameters[0] ?? null) === 'strict';
    }

    /**
     * @param  array{0?: string}  $parameters
     */
    public function b($attribute, $parameters)
    {
        return [$parameters[0] ?? $attribute.'_confirmation'];
    }

    /**
     * @param  array{0?: string}  $parameters
     */
    public function c($parameters)
    {
        return $parameters[0];
    }

    /**
     * @param  array{0: string, 1?: string}  $parameters
     */
    public function d($parameters)
    {
        return $parameters[0];
    }

    /**
     * @param  array{0: string}  $parameters
     */
    public function e($parameters)
    {
        return $parameters[0] ?? null;
    }
}
-----
<?php

final class Rules
{
    /**
     * @param  array{0?: 'strict'}  $parameters
     */
    public function a($parameters)
    {
        return ($parameters[0] ?? null) === 'strict';
    }

    /**
     * @param  array{0?: string}  $parameters
     */
    public function b($attribute, $parameters)
    {
        return [$parameters[0] ?? $attribute.'_confirmation'];
    }

    /**
     * @param  array{0?: string}  $parameters
     */
    public function c($parameters)
    {
        return $parameters[0];
    }

    /**
     * @param  array{0: string, 1?: string}  $parameters
     */
    public function d(array $parameters)
    {
        return $parameters[0];
    }

    /**
     * @param  array{0: string}  $parameters
     */
    public function e(array $parameters)
    {
        return $parameters[0] ?? null;
    }
}
