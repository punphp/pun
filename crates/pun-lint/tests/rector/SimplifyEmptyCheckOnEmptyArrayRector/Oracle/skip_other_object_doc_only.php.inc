<?php

final class Box
{
    /** @var array */
    public $docOnly = [];

    /** @var array */
    public static $staticDocOnly = [];

    public function set($v)
    {
        $this->docOnly = $v;
        self::$staticDocOnly = $v;
    }
}

final class OtherObjectDocOnly
{
    public function instance(Box $box): bool
    {
        return empty($box->docOnly);
    }

    public function static(): bool
    {
        return empty(Box::$staticDocOnly);
    }
}

trait InTrait
{
    /** @var array */
    private $list = [];

    public function direct(): bool
    {
        return empty($this->list);
    }
}
