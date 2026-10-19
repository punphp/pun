<?php

namespace App;

final class R
{
    protected function getExplicitKeys($attribute)
    {
        $pattern = str_replace('\*', '([^\.]*)', preg_quote($this->getPrimaryAttribute($attribute), '/'));

        if (preg_match('/^'.$pattern.'/', $attribute, $keys)) {
            array_shift($keys);

            return $keys;
        }

        return [];
    }

    protected function b($attribute, $pattern)
    {
        if (preg_match('/^'.$pattern.'/', $attribute, $keys)) {
            return $keys;
        }

        return [];
    }

    protected function c($attribute)
    {
        if (preg_match('/^a/', $attribute, $keys)) {
            array_shift($keys);

            return $keys;
        }

        return [];
    }

    protected function getPrimaryAttribute($attribute)
    {
        return $attribute;
    }
}
-----
<?php

namespace App;

final class R
{
    protected function getExplicitKeys($attribute)
    {
        $pattern = str_replace('\*', '([^\.]*)', preg_quote($this->getPrimaryAttribute($attribute), '/'));

        if (preg_match('/^'.$pattern.'/', $attribute, $keys)) {
            array_shift($keys);

            return $keys;
        }

        return [];
    }

    protected function b($attribute, $pattern)
    {
        if (preg_match('/^'.$pattern.'/', $attribute, $keys)) {
            return $keys;
        }

        return [];
    }

    protected function c($attribute): array
    {
        if (preg_match('/^a/', $attribute, $keys)) {
            array_shift($keys);

            return $keys;
        }

        return [];
    }

    protected function getPrimaryAttribute($attribute)
    {
        return $attribute;
    }
}
