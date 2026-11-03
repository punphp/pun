<?php

namespace DeepCopy\TypeMatcher;

class TypeMatcher
{
    /**
     * @var string
     */
    private $type;

    /**
     * @param string $type
     */
    public function __construct($type)
    {}

    /**
     * @param mixed $element
     *
     * @return boolean
     */
    public function matches($element)
    {}
}
