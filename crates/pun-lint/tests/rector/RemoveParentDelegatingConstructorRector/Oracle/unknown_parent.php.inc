<?php

namespace Pun\Oracle\UselessConstructor;

final class ExtendsUnknownParent extends \Pun\Oracle\Missing\UnknownBase
{
    public function __construct($value)
    {
        parent::__construct($value);
    }
}
