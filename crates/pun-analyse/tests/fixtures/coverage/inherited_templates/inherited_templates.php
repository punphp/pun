<?php

namespace CovInheritedTemplates;

/** @template T */
abstract class Base
{
    /** @var T */
    public $value;

    /** @return T */
    abstract public function get();

    /** @param T $v */
    public function set($v): void
    {
    }
}

/** @extends Base<int> */
class IntBox extends Base
{
    public $value = 'a';

    public function get()
    {
        return 'a';
    }
}

/** @template-covariant T of string */
interface Source
{
    /** @return T */
    public function read();
}

class Untagged implements Source
{
    public function read()
    {
        return 1;
    }
}

class Grand extends IntBox
{
    public function get()
    {
        return 1.5;
    }
}

/**
 * @template U
 * @extends Base<array<int, U>>
 */
class ListBox extends Base
{
    public function get()
    {
        return null;
    }
}
