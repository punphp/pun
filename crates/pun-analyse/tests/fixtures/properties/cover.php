<?php

namespace Cover;

class ParentPrivate
{
    private $secret;
}

class ChildPrivate extends ParentPrivate
{
    public function read()
    {
        return $this->secret;
    }

    public function destructure()
    {
        [, $this->missing] = [1, 2];
    }

    public function dynamic(string $name)
    {
        return isset(static::$$name);
    }

    public function exists()
    {
        if (property_exists(self::NAME, 'other')) {
            return self::$other;
        }
    }

    const NAME = 'x';
}

enum Suit
{
    case Hearts;

    public function read()
    {
        return $this->nope;
    }
}

readonly class Frozen
{
    public function read()
    {
        return $this->nope;
    }
}

trait Shared
{
    public function read()
    {
        return self::$nope . parent::$nope . static::$nope;
    }
}

class UsesShared extends ParentPrivate
{
    use Shared;
}

class Orphan extends \Missing\Base
{
    public function read()
    {
        return parent::$nope;
    }
}

final class Closed
{
    public function check()
    {
        return isset($this->nope);
    }
}
